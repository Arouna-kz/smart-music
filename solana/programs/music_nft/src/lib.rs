//! MusicNFT program for Solana
//! 
//! This program implements an ERC1155-like NFT standard for music editions with:
//! - Royalty distribution
//! - Multiple collaborators per edition
//! - Platform fee mechanism
//! - Supply management for each edition

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

declare_id!("YourProgramIDHere");

/// Basis points (1/100 of a percent) used for royalty calculations
const BASIS_POINTS: u16 = 10000;

/// MusicNFT program
#[program]
pub mod music_nft {
    use super::*;

    /// Initializes the MusicNFT program
    /// 
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * initial_uri - Base URI for token metadata
    /// * platform_fee - Platform fee percentage (basis points, max 1000 = 10%)
    pub fn initialize(
        ctx: Context<Initialize>,
        initial_uri: String,
        platform_fee: u16,
    ) -> Result<()> {
        require!(platform_fee <= 1000, ErrorCode::InvalidFee);
        
        let program_state = &mut ctx.accounts.program_state;
        program_state.platform_fee = platform_fee;
        program_state.base_uri = initial_uri;
        program_state.bump = *ctx.bumps.get("program_state").unwrap();
        
        Ok(())
    }

    /// Creates a new music edition
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * ipfs_hash - IPFS hash for edition metadata
    /// * max_supply - Maximum supply of tokens for this edition
    /// * collaborators - Array of royalty recipients and their percentages
    pub fn create_edition(
        ctx: Context<CreateEdition>,
        ipfs_hash: String,
        max_supply: u64,
        collaborators: Vec<RoyaltyRecipient>,
    ) -> Result<()> {
        require!(!ipfs_hash.is_empty(), ErrorCode::InvalidIpfsHash);
        require!(max_supply > 0, ErrorCode::InvalidSupply);
        require!(!collaborators.is_empty(), ErrorCode::NoCollaborators);
        
        let program_state = &mut ctx.accounts.program_state;
        let edition = &mut ctx.accounts.edition;
        
        // Generate new edition ID
        edition.id = program_state.edition_counter;
        program_state.edition_counter += 1;
        
        edition.ipfs_hash = ipfs_hash;
        edition.max_supply = max_supply;
        edition.minted_count = 0;
        edition.is_active = true;
        
        // Store royalty recipients and calculate total percentage
        let mut total_percentage = 0;
        for recipient in collaborators {
            require!(recipient.percentage > 0, ErrorCode::InvalidPercentage);
            total_percentage += recipient.percentage;
            edition.royalty_recipients.push(recipient);
        }
        
        require!(total_percentage <= BASIS_POINTS, ErrorCode::RoyaltiesTooHigh);
        edition.total_royalty_percentage = total_percentage;
        
        emit!(EditionCreated {
            edition_id: edition.id,
            ipfs_hash: edition.ipfs_hash.clone(),
            max_supply: edition.max_supply,
        });
        
        Ok(())
    }

    /// Mints tokens of a specific edition
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * edition_id - ID of the edition to mint
    /// * amount - Number of tokens to mint
    pub fn mint_edition(
        ctx: Context<MintEdition>,
        edition_id: u64,
        amount: u64,
    ) -> Result<()> {
        let edition = &mut ctx.accounts.edition;
        require!(edition.is_active, ErrorCode::EditionInactive);
        require!(edition.minted_count + amount <= edition.max_supply, ErrorCode::ExceedsMaxSupply);
        
        // Mint tokens to recipient
        edition.minted_count += amount;
        
        emit!(EditionMinted {
            edition_id: edition.id,
            to: ctx.accounts.recipient.key(),
            amount,
        });
        
        Ok(())
    }

    /// Distributes sale revenue to platform and collaborators
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * edition_id - ID of the edition generating revenue
    /// * amount - Total revenue amount to distribute
    pub fn distribute_sale_revenue(
        ctx: Context<DistributeRevenue>,
        edition_id: u64,
        amount: u64,
    ) -> Result<()> {
        let edition = &mut ctx.accounts.edition;
        let program_state = &ctx.accounts.program_state;
        
        // Calculate platform fee
        let fee_amount = amount
            .checked_mul(program_state.platform_fee.into())
            .unwrap()
            .checked_div(BASIS_POINTS.into())
            .unwrap();
        
        let remaining_amount = amount.checked_sub(fee_amount).unwrap();
        
        // Transfer platform fee
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.platform_wallet.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, fee_amount)?;
        
        // Distribute remaining to collaborators
        for recipient in &edition.royalty_recipients {
            let share = remaining_amount
                .checked_mul(recipient.percentage.into())
                .unwrap()
                .checked_div(edition.total_royalty_percentage.into())
                .unwrap();
                
            let cpi_accounts = Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: recipient.recipient.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            };
            let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
            token::transfer(cpi_ctx, share)?;
        }
        
        emit!(RevenueDistributed {
            edition_id,
            total_amount: amount,
            platform_fee_amount: fee_amount,
        });
        
        Ok(())
    }
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + ProgramState::MAX_SIZE)]
    pub program_state: Account<'info, ProgramState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for creating an edition
#[derive(Accounts)]
pub struct CreateEdition<'info> {
    #[account(mut)]
    pub program_state: Account<'info, ProgramState>,
    #[account(init, payer = authority, space = 8 + Edition::MAX_SIZE)]
    pub edition: Account<'info, Edition>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for minting an edition
#[derive(Accounts)]
pub struct MintEdition<'info> {
    #[account(mut)]
    pub edition: Account<'info, Edition>,
    #[account(mut)]
    pub recipient: AccountInfo<'info>,
    pub authority: Signer<'info>,
}

/// Accounts for distributing revenue
#[derive(Accounts)]
pub struct DistributeRevenue<'info> {
    #[account(mut)]
    pub edition: Account<'info, Edition>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    #[account(mut)]
    pub platform_wallet: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

/// Program state account
#[account]
pub struct ProgramState {
    pub bump: u8,
    pub edition_counter: u64,
    pub platform_fee: u16,
    pub base_uri: String,
}

impl ProgramState {
    pub const MAX_SIZE: usize = 1 + 8 + 2 + (4 + 200); // bump + counter + fee + URI (max 200 chars)
}

/// Edition account
#[account]
pub struct Edition {
    pub id: u64,
    pub ipfs_hash: String,
    pub max_supply: u64,
    pub minted_count: u64,
    pub total_royalty_percentage: u16,
    pub is_active: bool,
    pub royalty_recipients: Vec<RoyaltyRecipient>,
}

impl Edition {
    pub const MAX_SIZE: usize = 8 + (4 + 100) + 8 + 8 + 2 + 1 + (4 + 10 * RoyaltyRecipient::MAX_SIZE); // id + ipfs_hash (max 100) + supply + minted + royalty % + active + recipients (max 10)
}

/// Royalty recipient information
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RoyaltyRecipient {
    pub recipient: Pubkey,
    pub percentage: u16,
}

impl RoyaltyRecipient {
    pub const MAX_SIZE: usize = 32 + 2; // pubkey + percentage
}

/// Events
#[event]
pub struct EditionCreated {
    pub edition_id: u64,
    pub ipfs_hash: String,
    pub max_supply: u64,
}

#[event]
pub struct EditionMinted {
    pub edition_id: u64,
    pub to: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RevenueDistributed {
    pub edition_id: u64,
    pub total_amount: u64,
    pub platform_fee_amount: u64,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Platform fee must be <= 10% (1000 basis points)")]
    InvalidFee,
    #[msg("IPFS hash is required")]
    InvalidIpfsHash,
    #[msg("Supply must be positive")]
    InvalidSupply,
    #[msg("At least one collaborator required")]
    NoCollaborators,
    #[msg("Royalty percentage must be positive")]
    InvalidPercentage,
    #[msg("Total royalties cannot exceed 100%")]
    RoyaltiesTooHigh,
    #[msg("Edition is inactive")]
    EditionInactive,
    #[msg("Cannot exceed max supply")]
    ExceedsMaxSupply,
}