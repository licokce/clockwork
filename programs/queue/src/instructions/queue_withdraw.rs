use {crate::objects::*, anchor_lang::prelude::*};

/// Accounts required by the `queue_withdraw` instruction.
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct QueueWithdraw<'info> {
    /// The authority (owner) of the queue.
    #[account()]
    pub authority: Signer<'info>,

    /// The account to withdraw lamports to.
    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    /// The queue to be.
    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueWithdraw>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let queue = &mut ctx.accounts.queue;

    // Withdraw balance from queue to the pay_to account
    **queue.to_account_info().try_borrow_mut_lamports()? = queue
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .unwrap();
    **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
        .to_account_info()
        .lamports()
        .checked_add(amount)
        .unwrap();

    Ok(())
}
