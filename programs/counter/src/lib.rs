#![allow(deprecated)]
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("HKC2buFAdJAYih9ZBdG45CvdKdMpK4btNoUafuesdasu");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, max_count: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Validate max_count
        require! (max_count > 0, CounterError::InvalidMaxCount);
        require! (max_count <= 1_000_000, CounterError::MaxCountTooLarge);

        counter.count = 0;
        counter.max_count = max_count;
        counter.bump = ctx.bumps.counter;
        counter.authority = ctx.accounts.authority.key();

        emit!(CounterInitialized {
            authority: ctx.accounts.authority.key(),
            count: counter.count,
            max_count: counter.max_count,
        });

        msg!(
            "Counter initialized; authority = {}, count = {}, max_count = {}",
            ctx.accounts.authority.key(),
            counter.count,
            counter.max_count
        );
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Ensure we don't exceed max_count
        require!(
            counter.count < counter.max_count,
            CounterError::CountExceeded
        );

        counter.count = counter.count.checked_add(1)
            .ok_or(CounterError::ArithmeticOverflow)?;

        let previous_count = counter.count - 1;
        emit!(CounterUpdated {
            authority: ctx.accounts.authority.key(),
            previous_count,
            new_count: counter.count,
            max_reached: counter.count == counter.max_count,
        });

        if counter.count == counter.max_count {
            msg!(
                "Counter has reached its maximum value of {}",
                counter.max_count
            );
        } else {
            msg!(
                "Counter incremented to: {}", counter.count
            );
        }
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Ensure we don't go below zero
        require!(counter.count > 0, CounterError::Underflow);

        counter.count = counter.count.checked_sub(1)
            .ok_or(CounterError::ArithmeticOverflow)?;

        let previous_count = counter.count + 1;
        emit!(CounterUpdated {
            authority: ctx.accounts.authority.key(),
            previous_count,
            new_count: counter.count,
            max_reached: false,
        });

        msg!(
            "Counter decremented to: {}", counter.count
        );
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;

        // Only authority can reset
        require!(
            ctx.accounts.authority.key() == counter.authority,
            CounterError::Unauthorized
        );

        let previous_count = counter.count + 1;
        counter.count = 0;

        emit!(CounterReset {
            authority: ctx.accounts.authority.key(),
            previous_count,
            new_count: 0,
        });

        msg!(
            "Counter reset to 0 by authority"
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 8 + 1 + 32, // discriminator + count + max_count + bump + authority pubkey
        seeds = [b"counter", authority.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        has_one = authority @ CounterError::Unauthorized,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority @ CounterError::Unauthorized,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
    )]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority @ CounterError::Unauthorized,
        seeds = [b"counter", counter.authority.as_ref()],
        bump = counter.bump,
        close = authority, // reclaim rent on close
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub count: u64,
    pub max_count: u64,
    pub bump: u8,
    pub authority: Pubkey,
}

#[event]
pub struct CounterInitialized {
    pub authority: Pubkey,
    pub count: u64,
    pub max_count: u64,
}

#[event]
pub struct CounterUpdated {
    pub authority: Pubkey,
    pub previous_count: u64,
    pub new_count: u64,
    pub max_reached: bool,
}

#[event]
pub struct CounterReset {
    pub authority: Pubkey,
    pub previous_count: u64,
    pub new_count: u64,
}

#[error_code]
pub enum CounterError {
    #[msg("The counter has reached its maximum value.")]
    CountExceeded,
    #[msg("Underflow: Counter cannot go below zero.")]
    Underflow,
    #[msg("Overflow: Arithmetic operation overflowed.")]
    ArithmeticOverflow,
    #[msg("Unauthorized: Only the authority can perform this action.")]
    Unauthorized,
    #[msg("Invalid max_count: must be greater than zero.")]
    InvalidMaxCount,
    #[msg("Max count exceeds the allowed limit of 1,000,000.")]
    MaxCountTooLarge,
}





