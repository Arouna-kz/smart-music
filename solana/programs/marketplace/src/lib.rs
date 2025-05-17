//! NFT Marketplace program for Solana
//!
//! A decentralized marketplace supporting both fixed-price sales and auctions
//! for tokens, with royalty distribution and platform fee mechanism.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

declare_id!("YourProgramIDHere");

/// Basis points (1/100 of a percent) used for fee calculations
const BASIS_POINTS: u16 = 10000;

/// NFT Marketplace program
#[program]
pub mod nft_marketplace {
    use super::*;

    /// Initializes the marketplace
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * platform_fee - Platform fee percentage (basis points, max 1000 = 10%)
    /// * secondary_fee - Additional fee for secondary sales (basis points, max 1000 = 10%)
    pub fn initialize(
        ctx: Context<Initialize>,
        platform_fee: u16,
        secondary_fee: u16,
    ) -> Result<()> {
        require!(platform_fee <= 1000, ErrorCode::FeeTooHigh);
        require!(secondary_fee <= 1000, ErrorCode::FeeTooHigh);
        
        let marketplace = &mut ctx.accounts.marketplace;
        marketplace.platform_fee = platform_fee;
        marketplace.secondary_fee = secondary_fee;
        marketplace.bump = *ctx.bumps.get("marketplace").unwrap();
        
        Ok(())
    }

    /// Lists an NFT for sale or auction
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * nft_mint - Mint address of the NFT
    /// * price - Fixed price for direct sale (or starting bid for auction)
    /// * amount - Number of tokens to list
    /// * is_auction - Whether to create an auction listing
    /// * auction_duration - Duration of auction in seconds (if applicable)
    /// * custom_fee - Optional custom platform fee (overrides defaults)
    pub fn list_nft(
        ctx: Context<ListNft>,
        nft_mint: Pubkey,
        price: u64,
        amount: u64,
        is_auction: bool,
        auction_duration: u64,
        custom_fee: Option<u16>,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        let marketplace = &ctx.accounts.marketplace;
        
        // Determine fee (custom or default)
        let fee = custom_fee.unwrap_or(marketplace.platform_fee);
        require!(fee <= 1000, ErrorCode::FeeTooHigh);
        
        listing.nft_mint = nft_mint;
        listing.price = price;
        listing.seller = *ctx.accounts.seller.key;
        listing.is_auction = is_auction;
        listing.auction_end = if is_auction {
            Clock::get()?.unix_timestamp.checked_add(auction_duration as i64).unwrap()
        } else {
            0
        };
        listing.highest_bid = 0;
        listing.highest_bidder = None;
        listing.amount = amount;
        listing.platform_fee = fee;
        
        // Transfer NFT to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        
        emit!(NftListed {
            nft_mint,
            seller: listing.seller,
            price,
            amount,
            is_auction,
        });
        
        Ok(())
    }

    /// Purchases an NFT at fixed price
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * listing_id - ID of the listing to purchase from
    /// * amount - Number of tokens to purchase
    pub fn buy_nft(
        ctx: Context<BuyNft>,
        listing_id: u64,
        amount: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.price > 0, ErrorCode::NotForSale);
        require!(!listing.is_auction, ErrorCode::UseBidFunction);
        require!(amount > 0 && amount <= listing.amount, ErrorCode::InvalidAmount);
        
        let total_price = listing.price.checked_mul(amount).unwrap();
        
        // Process payment and distribution
        process_purchase(
            ctx.accounts.into(),
            listing.nft_mint,
            total_price,
            amount,
            listing.platform_fee,
        )?;
        
        // Update listing
        listing.amount = listing.amount.checked_sub(amount).unwrap();
        if listing.amount == 0 {
            // Close listing if all items sold
            ctx.accounts.listing.close(ctx.accounts.buyer.to_account_info())?;
        }
        
        emit!(NftSold {
            nft_mint: listing.nft_mint,
            buyer: ctx.accounts.buyer.key(),
            price: listing.price,
            amount,
        });
        
        Ok(())
    }

    /// Places a bid in an active auction
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * listing_id - ID of the auction listing
    /// * bid_amount - Bid amount
    pub fn place_bid(
        ctx: Context<PlaceBid>,
        listing_id: u64,
        bid_amount: u64,
    ) -> Result<()> {
        let listing = &mut ctx.accounts.listing;
        require!(listing.is_auction, ErrorCode::NotAuction);
        require!(Clock::get()?.unix_timestamp < listing.auction_end, ErrorCode::AuctionEnded);
        require!(bid_amount > listing.highest_bid, ErrorCode::BidTooLow);
        
        // Refund previous bid if any
        if let Some(prev_bidder) = listing.highest_bidder {
            let prev_bid = listing.highest_bid;
            let cpi_accounts = Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.prev_bidder_token_account.to_account_info(),
                authority: ctx.accounts.marketplace.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, prev_bid)?;
        }
        
        // Record new bid
        listing.highest_bid = bid_amount;
        listing.highest_bidder = Some(*ctx.accounts.bidder.key);
        
        // Transfer bid amount to escrow
        let cpi_accounts = Transfer {
            from: ctx.accounts.bidder_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.bidder.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, bid_amount)?;
        
        emit!(BidPlaced {
            nft_mint: listing.nft_mint,
            bidder: ctx.accounts.bidder.key(),
            amount: bid_amount,
        });
        
        Ok(())
    }

    /// Finalizes an ended auction
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * listing_id - ID of the auction listing
    pub fn finalize_auction(
        ctx: Context<FinalizeAuction>,
        listing_id: u64,
    ) -> Result<()> {
        let listing = &ctx.accounts.listing;
        require!(listing.is_auction, ErrorCode::NotAuction);
        require!(Clock::get()?.unix_timestamp >= listing.auction_end, ErrorCode::AuctionNotEnded);
        require!(listing.highest_bid > 0, ErrorCode::NoBids);
        
        let buyer = listing.highest_bidder.ok_or(ErrorCode::NoBids)?;
        
        // Process purchase
        process_purchase(
            ctx.accounts.into(),
            listing.nft_mint,
            listing.highest_bid,
            listing.amount,
            listing.platform_fee,
        )?;
        
        emit!(NftSold {
            nft_mint: listing.nft_mint,
            buyer,
            price: listing.highest_bid,
            amount: listing.amount,
        });
        
        // Close listing
        ctx.accounts.listing.close(ctx.accounts.buyer.to_account_info())?;
        
        Ok(())
    }

    /// Internal function to process purchase and distribute funds
    fn process_purchase(
        ctx: PurchaseContext,
        nft_mint: Pubkey,
        sale_price: u64,
        amount: u64,
        platform_fee: u16,
    ) -> Result<()> {
        // Calculate platform fee
        let fee_amount = sale_price
            .checked_mul(platform_fee.into())
            .unwrap()
            .checked_div(BASIS_POINTS.into())
            .unwrap();
        
        let remaining_amount = sale_price.checked_sub(fee_amount).unwrap();
        
        // Transfer NFT to buyer
        let cpi_accounts = Transfer {
            from: ctx.escrow_token_account.to_account_info(),
            to: ctx.buyer_token_account.to_account_info(),
            authority: ctx.marketplace.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        
        // Transfer platform fee
        let cpi_accounts = Transfer {
            from: ctx.escrow_token_account.to_account_info(),
            to: ctx.platform_wallet.to_account_info(),
            authority: ctx.marketplace.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, fee_amount)?;
        
        // Transfer remaining to seller
        let cpi_accounts = Transfer {
            from: ctx.escrow_token_account.to_account_info(),
            to: ctx.seller_token_account.to_account_info(),
            authority: ctx.marketplace.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, remaining_amount)?;
        
        Ok(())
    }
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Marketplace::MAX_SIZE)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for listing an NFT
#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(init, payer = seller, space = 8 + Listing::MAX_SIZE)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

/// Accounts for buying an NFT
#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub platform_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Accounts for placing a bid
#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub bidder: Signer<'info>,
    #[account(mut)]
    pub bidder_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub prev_bidder_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Accounts for finalizing an auction
#[derive(Accounts)]
pub struct FinalizeAuction<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, Marketplace>,
    #[account(mut)]
    pub listing: Account<'info, Listing>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub platform_wallet: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Marketplace state account
#[account]
pub struct Marketplace {
    pub bump: u8,
    pub platform_fee: u16,
    pub secondary_fee: u16,
}

impl Marketplace {
    pub const MAX_SIZE: usize = 1 + 2 + 2; // bump + platform_fee + secondary_fee
}

/// Listing account
#[account]
pub struct Listing {
    pub nft_mint: Pubkey,
    pub price: u64,
    pub seller: Pubkey,
    pub is_auction: bool,
    pub auction_end: i64,
    pub highest_bid: u64,
    pub highest_bidder: Option<Pubkey>,
    pub amount: u64,
    pub platform_fee: u16,
}

impl Listing {
    pub const MAX_SIZE: usize = 32 + 8 + 32 + 1 + 8 + 8 + (1 + 32) + 8 + 2;
}

/// Events
#[event]
pub struct NftListed {
    pub nft_mint: Pubkey,
    pub seller: Pubkey,
    pub price: u64,
    pub amount: u64,
    pub is_auction: bool,
}

#[event]
pub struct NftSold {
    pub nft_mint: Pubkey,
    pub buyer: Pubkey,
    pub price: u64,
    pub amount: u64,
}

#[event]
pub struct BidPlaced {
    pub nft_mint: Pubkey,
    pub bidder: Pubkey,
    pub amount: u64,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Fee must be <= 10% (1000 basis points)")]
    FeeTooHigh,
    #[msg("NFT is not for sale")]
    NotForSale,
    #[msg("Use bid function for auctions")]
    UseBidFunction,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Not an auction")]
    NotAuction,
    #[msg("Auction has ended")]
    AuctionEnded,
    #[msg("Bid amount too low")]
    BidTooLow,
    #[msg("Auction not ended yet")]
    AuctionNotEnded,
    #[msg("No bids placed")]
    NoBids,
}