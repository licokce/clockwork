//! This program allows users to create transaction queues on Solana. Queues are dynamic, long-running
//! transaction threads that can persist across blocks and even run indefinitely. Developers can use queues
//! to schedule transactions and automate smart-contracts without relying on centralized infrastructure.

pub mod errors;
pub mod objects;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::*;
use instructions::*;
use objects::*;

declare_id!("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv");

/// Program for creating transaction queues on Solana.
#[program]
pub mod queue_program {
    use super::*;

    /// Cranks a transaction queue.
    pub fn queue_crank(ctx: Context<QueueCrank>) -> Result<()> {
        queue_crank::handler(ctx)
    }

    /// Creates a new transaction queue.
    pub fn queue_create(
        ctx: Context<QueueCreate>,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        queue_create::handler(ctx, id, kickoff_instruction, trigger)
    }

    /// Closes an existing queue account and returns the lamports to the owner.
    pub fn queue_delete(ctx: Context<QueueDelete>) -> Result<()> {
        queue_delete::handler(ctx)
    }

    /// Kicks off a queue if its trigger condition is active.
    pub fn queue_kickoff(ctx: Context<QueueKickoff>, data_hash: Option<u64>) -> Result<()> {
        queue_kickoff::handler(ctx, data_hash)
    }

    /// Pauses an active queue.
    pub fn queue_pause(ctx: Context<QueuePause>) -> Result<()> {
        queue_pause::handler(ctx)
    }

    /// Resumes a paused queue.
    pub fn queue_resume(ctx: Context<QueueResume>) -> Result<()> {
        queue_resume::handler(ctx)
    }

    /// Resumes a paused queue.
    pub fn queue_stop(ctx: Context<QueueStop>) -> Result<()> {
        queue_stop::handler(ctx)
    }

    /// Allows an owner to update the mutable properties of a queue.
    pub fn queue_update(ctx: Context<QueueUpdate>, settings: QueueSettings) -> Result<()> {
        queue_update::handler(ctx, settings)
    }

    /// Allows an owner to withdraw from a queue's lamport balance.
    pub fn queue_withdraw(ctx: Context<QueueWithdraw>, amount: u64) -> Result<()> {
        queue_withdraw::handler(ctx, amount)
    }
}
