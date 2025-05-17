//! LicenseManager program for Solana
//!
//! A smart contract for managing NFT licenses, tracking usage, and collecting royalties

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

declare_id!("YourProgramIDHere");

/// LicenseManager program
#[program]
pub mod license_manager {
    use super::*;

    /// Initializes the LicenseManager
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let manager = &mut ctx.accounts.manager;
        manager.bump = *ctx.bumps.get("manager").unwrap();
        manager.license_count = 0;
        
        Ok(())
    }

    /// Issues a new license for NFT usage
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * token_id - NFT token ID to license
    /// * licensee - Address receiving license
    /// * duration_days - License duration in days
    /// * territory - Geographic territory rights
    /// * usage_type - Intended use classification
    /// * royalty_rate - Royalty percentage (basis points)
    /// * quantity - Number of tokens being licensed
    pub fn issue_license(
        ctx: Context<IssueLicense>,
        token_id: u64,
        licensee: Pubkey,
        duration_days: u64,
        territory: String,
        usage_type: String,
        royalty_rate: u16,
        quantity: u64,
    ) -> Result<()> {
        require!(royalty_rate <= BASIS_POINTS, ErrorCode::RoyaltyTooHigh);
        
        let manager = &mut ctx.accounts.manager;
        let license = &mut ctx.accounts.license;
        
        manager.license_count += 1;
        
        license.id = manager.license_count;
        license.token_id = token_id;
        license.licensee = licensee;
        license.start_date = Clock::get()?.unix_timestamp;
        license.end_date = Clock::get()?.unix_timestamp
            .checked_add((duration_days * 86400) as i64)
            .unwrap();
        license.territory = territory;
        license.usage_type = usage_type;
        license.royalty_rate = royalty_rate;
        license.is_active = true;
        license.quantity = quantity;
        
        emit!(LicenseIssued {
            license_id: license.id,
            token_id: license.token_id,
            licensee: license.licensee,
            royalty_rate: license.royalty_rate,
        });
        
        Ok(())
    }

    /// Records usage and processes royalty payment
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * license_id - ID of license being used
    /// * usage_amount - Base amount for royalty calculation
    pub fn record_usage(
        ctx: Context<RecordUsage>,
        license_id: u64,
        usage_amount: u64,
    ) -> Result<()> {
        let license = &ctx.accounts.license;
        require!(license.is_active, ErrorCode::LicenseInactive);
        require!(Clock::get()?.unix_timestamp <= license.end_date, ErrorCode::LicenseExpired);
        
        let royalty_amount = usage_amount
            .checked_mul(license.royalty_rate.into())
            .unwrap()
            .checked_div(BASIS_POINTS.into())
            .unwrap();
        
        // Transfer royalty payment
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info(),
            to: ctx.accounts.royalty_recipient.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, royalty_amount)?;
        
        emit!(RoyaltyPaid {
            license_id,
            amount: royalty_amount,
        });
        
        Ok(())
    }
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + LicenseManager::MAX_SIZE)]
    pub manager: Account<'info, LicenseManager>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for issuing a license
#[derive(Accounts)]
pub struct IssueLicense<'info> {
    #[account(mut)]
    pub manager: Account<'info, LicenseManager>,
    #[account(init, payer = authority, space = 8 + License::MAX_SIZE)]
    pub license: Account<'info, License>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for recording usage
#[derive(Accounts)]
pub struct RecordUsage<'info> {
    #[account(mut)]
    pub license: Account<'info, License>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub royalty_recipient: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

/// LicenseManager state account
#[account]
pub struct LicenseManager {
    pub bump: u8,
    pub license_count: u64,
}

impl LicenseManager {
    pub const MAX_SIZE: usize = 1 + 8; // bump + license_count
}

/// License account
#[account]
pub struct License {
    pub id: u64,
    pub token_id: u64,
    pub licensee: Pubkey,
    pub start_date: i64,
    pub end_date: i64,
    pub territory: String,
    pub usage_type: String,
    pub royalty_rate: u16,
    pub is_active: bool,
    pub quantity: u64,
}

impl License {
    pub const MAX_SIZE: usize = 8 + 8 + 32 + 8 + 8 + (4 + 100) + (4 + 50) + 2 + 1 + 8;
}

/// Events
#[event]
pub struct LicenseIssued {
    pub license_id: u64,
    pub token_id: u64,
    pub licensee: Pubkey,
    pub royalty_rate: u16,
}

#[event]
pub struct RoyaltyPaid {
    pub license_id: u64,
    pub amount: u64,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Royalty rate cannot exceed 100%")]
    RoyaltyTooHigh,
    #[msg("License is not active")]
    LicenseInactive,
    #[msg("License has expired")]
    LicenseExpired,
}