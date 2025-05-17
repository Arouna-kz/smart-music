//! CrowdfundingDAO program for Solana
//!
//! A DAO-powered crowdfunding platform where artists can raise funds for creative projects
//! and investors receive governance tokens and future royalties.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

declare_id!("YourProgramIDHere");

/// Basis points (1/100 of a percent) used for royalty calculations
const BASIS_POINTS: u16 = 10000;

/// CrowdfundingDAO program
#[program]
pub mod crowdfunding_dao {
    use super::*;

    /// Initializes the crowdfunding DAO
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        dao.bump = *ctx.bumps.get("dao").unwrap();
        dao.campaign_count = 0;
        
        Ok(())
    }

    /// Creates a new crowdfunding campaign
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * project_name - Name of creative project
    /// * description - Detailed project description
    /// * image - URL of project image
    /// * funding_goal - Target funding amount
    /// * duration_days - Campaign duration in days
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        project_name: String,
        description: String,
        image: String,
        funding_goal: u64,
        duration_days: u64,
    ) -> Result<()> {
        let dao = &mut ctx.accounts.dao;
        let campaign = &mut ctx.accounts.campaign;
        
        dao.campaign_count += 1;
        
        campaign.id = dao.campaign_count;
        campaign.artist = *ctx.accounts.artist.key;
        campaign.project_name = project_name;
        campaign.description = description;
        campaign.image = image;
        campaign.funding_goal = funding_goal;
        campaign.deadline = Clock::get()?.unix_timestamp
            .checked_add((duration_days * 86400) as i64)
            .unwrap();
        campaign.is_approved = false;
        campaign.is_completed = false;
        
        emit!(CampaignCreated {
            campaign_id: campaign.id,
            artist: campaign.artist,
            project_name: campaign.project_name.clone(),
            description: campaign.description.clone(),
            image: campaign.image.clone(),
            funding_goal: campaign.funding_goal,
            deadline: campaign.deadline,
        });
        
        Ok(())
    }

    /// Approves a campaign (DAO governance)
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * campaign_id - ID of campaign to approve
    pub fn approve_campaign(ctx: Context<ApproveCampaign>, campaign_id: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(!campaign.is_approved, ErrorCode::AlreadyApproved);
        
        // Verify governance token balance
        let governance_balance = ctx.accounts.governance_token_account.amount;
        require!(governance_balance > 0, ErrorCode::NoGovernanceTokens);
        
        campaign.is_approved = true;
        
        emit!(CampaignApproved {
            campaign_id: campaign.id,
        });
        
        Ok(())
    }

    /// Contributes ETH to a campaign
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * campaign_id - ID of campaign to fund
    /// * amount - Contribution amount in lamports
    pub fn contribute_eth(ctx: Context<ContributeEth>, campaign_id: u64, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(campaign.is_approved, ErrorCode::NotApproved);
        require!(Clock::get()?.unix_timestamp <= campaign.deadline, ErrorCode::DeadlinePassed);
        require!(amount > 0, ErrorCode::InvalidAmount);
        
        // Record contribution
        campaign.amount_raised_eth = campaign.amount_raised_eth.checked_add(amount).unwrap();
        
        // Add investor if not already
        if !campaign.investors.contains(ctx.accounts.investor.key) {
            campaign.investors.push(*ctx.accounts.investor.key);
        }
        
        campaign.contributions_eth.insert(*ctx.accounts.investor.key, amount);
        
        // Mint governance tokens (1 token per lamport)
        let cpi_accounts = Transfer {
            from: ctx.accounts.dao_governance_account.to_account_info(),
            to: ctx.accounts.investor_governance_account.to_account_info(),
            authority: ctx.accounts.dao.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
        token::transfer(cpi_ctx, amount)?;
        
        emit!(ContributionMadeEth {
            campaign_id,
            investor: ctx.accounts.investor.key(),
            amount,
        });
        
        Ok(())
    }

    /// Marks a campaign as completed
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * campaign_id - ID of campaign to complete
    pub fn complete_campaign(ctx: Context<CompleteCampaign>, campaign_id: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        require!(campaign.artist == *ctx.accounts.artist.key, ErrorCode::NotArtist);
        require!(Clock::get()?.unix_timestamp > campaign.deadline, ErrorCode::DeadlineNotPassed);
        require!(!campaign.is_completed, ErrorCode::AlreadyCompleted);
        
        campaign.is_completed = true;
        
        emit!(CampaignCompleted {
            campaign_id,
            amount_raised_eth: campaign.amount_raised_eth,
            amount_raised_tokens: campaign.amount_raised_tokens,
        });
        
        Ok(())
    }

    /// Records revenue and distributes royalties
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * campaign_id - ID of campaign
    /// * amount - Revenue amount to distribute
    /// * distribute_in_tokens - Whether to pay in tokens (false for ETH)
    pub fn record_revenue(
        ctx: Context<RecordRevenue>,
        campaign_id: u64,
        amount: u64,
        distribute_in_tokens: bool,
    ) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        let royalty = &mut ctx.accounts.royalty;
        
        require!(campaign.is_completed, ErrorCode::NotCompleted);
        
        royalty.total_revenue = royalty.total_revenue.checked_add(amount).unwrap();
        let royalty_amount = amount
            .checked_mul(royalty.royalty_percentage.into())
            .unwrap()
            .checked_div(BASIS_POINTS.into())
            .unwrap();
        
        // Distribute to all investors
        for investor in &campaign.investors {
            let total_contribution = campaign.contributions_eth.get(investor).unwrap_or(0)
                + campaign.contributions_tokens.get(investor).unwrap_or(0);
            
            let investor_share = total_contribution
                .checked_mul(royalty_amount)
                .unwrap()
                .checked_div(campaign.amount_raised_eth + campaign.amount_raised_tokens)
                .unwrap();
            
            if distribute_in_tokens {
                let cpi_accounts = Transfer {
                    from: ctx.accounts.dao_token_account.to_account_info(),
                    to: ctx.accounts.investor_token_account.to_account_info(),
                    authority: ctx.accounts.dao.to_account_info(),
                };
                let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
                token::transfer(cpi_ctx, investor_share)?;
                
                emit!(RoyaltyDistributedTokens {
                    campaign_id,
                    investor: *investor,
                    amount: investor_share,
                });
            } else {
                **ctx.accounts.dao.to_account_info().try_borrow_mut_lamports()? -= investor_share;
                **ctx.accounts.investor.to_account_info().try_borrow_mut_lamports()? += investor_share;
                
                emit!(RoyaltyDistributedEth {
                    campaign_id,
                    investor: *investor,
                    amount: investor_share,
                });
            }
        }
        
        emit!(RevenueRecorded {
            campaign_id,
            amount,
        });
        
        Ok(())
    }
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Dao::MAX_SIZE)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for creating a campaign
#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(init, payer = artist, space = 8 + Campaign::MAX_SIZE)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub artist: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for approving a campaign
#[derive(Accounts)]
pub struct ApproveCampaign<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub governance_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Accounts for contributing ETH
#[derive(Accounts)]
pub struct ContributeEth<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub investor: Signer<'info>,
    #[account(mut)]
    pub dao_governance_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub investor_governance_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// Accounts for completing a campaign
#[derive(Accounts)]
pub struct CompleteCampaign<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub artist: Signer<'info>,
}

/// Accounts for recording revenue
#[derive(Accounts)]
pub struct RecordRevenue<'info> {
    #[account(mut)]
    pub dao: Account<'info, Dao>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub royalty: Account<'info, Royalty>,
    #[account(mut)]
    pub investor: AccountInfo<'info>,
    #[account(mut)]
    pub investor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub dao_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

/// DAO state account
#[account]
pub struct Dao {
    pub bump: u8,
    pub campaign_count: u64,
}

impl Dao {
    pub const MAX_SIZE: usize = 1 + 8; // bump + campaign_count
}

/// Campaign account
#[account]
pub struct Campaign {
    pub id: u64,
    pub artist: Pubkey,
    pub project_name: String,
    pub description: String,
    pub image: String,
    pub funding_goal: u64,
    pub amount_raised_eth: u64,
    pub amount_raised_tokens: u64,
    pub deadline: i64,
    pub is_approved: bool,
    pub is_completed: bool,
    pub investors: Vec<Pubkey>,
    pub contributions_eth: BTreeMap<Pubkey, u64>,
    pub contributions_tokens: BTreeMap<Pubkey, u64>,
}

impl Campaign {
    pub const MAX_SIZE: usize = 8 + 32 + (4 + 100) + (4 + 500) + (4 + 200) + 8 + 8 + 8 + 8 + 1 + 1 + (4 + 100 * 32) + (4 + 100 * (32 + 8)) * 2;
}

/// Royalty account
#[account]
pub struct Royalty {
    pub campaign_id: u64,
    pub total_revenue: u64,
    pub royalty_percentage: u16,
}

impl Royalty {
    pub const MAX_SIZE: usize = 8 + 8 + 2;
}

/// Events
#[event]
pub struct CampaignCreated {
    pub campaign_id: u64,
    pub artist: Pubkey,
    pub project_name: String,
    pub description: String,
    pub image: String,
    pub funding_goal: u64,
    pub deadline: i64,
}

#[event]
pub struct CampaignApproved {
    pub campaign_id: u64,
}

#[event]
pub struct ContributionMadeEth {
    pub campaign_id: u64,
    pub investor: Pubkey,
    pub amount: u64,
}

#[event]
pub struct CampaignCompleted {
    pub campaign_id: u64,
    pub amount_raised_eth: u64,
    pub amount_raised_tokens: u64,
}

#[event]
pub struct RevenueRecorded {
    pub campaign_id: u64,
    pub amount: u64,
}

#[event]
pub struct RoyaltyDistributedEth {
    pub campaign_id: u64,
    pub investor: Pubkey,
    pub amount: u64,
}

#[event]
pub struct RoyaltyDistributedTokens {
    pub campaign_id: u64,
    pub investor: Pubkey,
    pub amount: u64,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Campaign is already approved")]
    AlreadyApproved,
    #[msg("You must hold governance tokens to approve")]
    NoGovernanceTokens,
    #[msg("Campaign is not approved")]
    NotApproved,
    #[msg("Campaign deadline has passed")]
    DeadlinePassed,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Only the artist can complete the campaign")]
    NotArtist,
    #[msg("Campaign deadline has not passed")]
    DeadlineNotPassed,
    #[msg("Campaign is already completed")]
    AlreadyCompleted,
    #[msg("Campaign is not completed")]
    NotCompleted,
}