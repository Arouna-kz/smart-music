//! StreamingRevenue program for Solana
//!
//! A system to track music streams and distribute revenue to collaborators

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

declare_id!("YourProgramIDHere");

/// StreamingRevenue program
#[program]
pub mod streaming_revenue {
    use super::*;

    /// Initializes the streaming revenue contract
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * streams_per_payout - Streams required per payout
    /// * payout_amount - Payment amount per streams_per_payout (in lamports)
    pub fn initialize(
        ctx: Context<Initialize>,
        streams_per_payout: u64,
        payout_amount: u64,
    ) -> Result<()> {
        let stream = &mut ctx.accounts.stream;
        stream.streams_per_payout = streams_per_payout;
        stream.payout_amount = payout_amount;
        stream.bump = *ctx.bumps.get("stream").unwrap();
        
        Ok(())
    }

    /// Records a stream and processes payout if threshold reached
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * track_id - ID of the track being streamed
    pub fn record_stream(ctx: Context<RecordStream>, track_id: u64) -> Result<()> {
        let track = &mut ctx.accounts.track;
        track.stream_count = track.stream_count.checked_add(1).unwrap();
        
        if track.stream_count.checked_sub(track.last_payout_at).unwrap() >= track.streams_per_payout {
            process_payout(
                ctx.accounts.into(),
                track_id,
            )?;
        }
        
        emit!(StreamRecorded {
            track_id,
            listener: ctx.accounts.listener.key(),
        });
        
        Ok(())
    }

    /// Adds a new track with collaborators and shares
    ///
    /// # Arguments
    /// * ctx - Context containing program accounts
    /// * track_id - ID to associate with track
    /// * collaborators - Array of contributor addresses
    /// * shares - Array of revenue shares (basis points)
    pub fn add_track(
        ctx: Context<AddTrack>,
        track_id: u64,
        collaborators: Vec<Pubkey>,
        shares: Vec<u16>,
    ) -> Result<()> {
        require!(collaborators.len() == shares.len(), ErrorCode::LengthMismatch);
        
        let total_shares: u16 = shares.iter().sum();
        require!(total_shares == BASIS_POINTS, ErrorCode::InvalidShares);
        
        let track = &mut ctx.accounts.track;
        track.id = track_id;
        track.collaborators = collaborators;
        track.shares = shares;
        
        Ok(())
    }

    /// Internal function to process payouts
    fn process_payout(ctx: PayoutContext, track_id: u64) -> Result<()> {
        let track = &ctx.accounts.track;
        
        let streams_since_last = track.stream_count.checked_sub(track.last_payout_at).unwrap();
        let payout_count = streams_since_last.checked_div(track.streams_per_payout).unwrap();
        let total_payout = payout_count.checked_mul(track.payout_amount).unwrap();
        
        let mut amounts = Vec::new();
        for (i, collaborator) in track.collaborators.iter().enumerate() {
            let amount = total_payout
                .checked_mul(track.shares[i].into())
                .unwrap()
                .checked_div(BASIS_POINTS.into())
                .unwrap();
            
            amounts.push(amount);
            
            // Transfer share to collaborator
            **ctx.accounts.dao.to_account_info().try_borrow_mut_lamports()? -= amount;
            **collaborator.to_account_info().try_borrow_mut_lamports()? += amount;
        }
        
        // Update last payout
        ctx.accounts.track.last_payout_at = track.stream_count;
        
        emit!(PayoutCompleted {
            track_id,
            amount: total_payout,
            recipients: track.collaborators.clone(),
            amounts,
        });
        
        Ok(())
    }
}

/// Accounts for initialization
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + StreamingRevenue::MAX_SIZE)]
    pub stream: Account<'info, StreamingRevenue>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts for recording a stream
#[derive(Accounts)]
pub struct RecordStream<'info> {
    #[account(mut)]
    pub stream: Account<'info, StreamingRevenue>,
    #[account(mut)]
    pub track: Account<'info, Track>,
    pub listener: Signer<'info>,
}

/// Accounts for adding a track
#[derive(Accounts)]
pub struct AddTrack<'info> {
    #[account(mut)]
    pub stream: Account<'info, StreamingRevenue>,
    #[account(init, payer = authority, space = 8 + Track::MAX_SIZE)]
    pub track: Account<'info, Track>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// StreamingRevenue state account
#[account]
pub struct StreamingRevenue {
    pub bump: u8,
    pub streams_per_payout: u64,
    pub payout_amount: u64,
}

impl StreamingRevenue {
    pub const MAX_SIZE: usize = 1 + 8 + 8; // bump + streams_per_payout + payout_amount
}

/// Track account
#[account]
pub struct Track {
    pub id: u64,
    pub stream_count: u64,
    pub last_payout_at: u64,
    pub streams_per_payout: u64,
    pub collaborators: Vec<Pubkey>,
    pub shares: Vec<u16>,
}

impl Track {
    pub const MAX_SIZE: usize = 8 + 8 + 8 + 8 + (4 + 10 * 32) + (4 + 10 * 2); // id + count + last_payout + per_payout + collaborators (max 10) + shares (max 10)
}

/// Events
#[event]
pub struct StreamRecorded {
    pub track_id: u64,
    pub listener: Pubkey,
}

#[event]
pub struct PayoutCompleted {
    pub track_id: u64,
    pub amount: u64,
    pub recipients: Vec<Pubkey>,
    pub amounts: Vec<u64>,
}

/// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Collaborators and shares length mismatch")]
    LengthMismatch,
    #[msg("Total shares must equal 10000 (100%)")]
    InvalidShares,
}