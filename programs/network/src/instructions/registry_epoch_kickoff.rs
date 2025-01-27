use {
    crate::objects::*,
    anchor_lang::prelude::*,
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

// DONE Payout yield.
//      Transfer lamports collected by Fee accounts to Delegation accounts based on the stake balance distributions of the current Epoch's SnapshotEntries.

// DONE Process unstake requests.
//      For each "unstake request" transfer tokens from the Worker stake account to the Delegation authority's token account.
//      Decrement the Delegation's stake balance by the amount unstaked.

// DONE Lock delegated stakes.
//      Transfer tokens from the Delegation's stake account to the Worker's stake account.
//      Increment the Delegation's stake balance by the amount moved.

// DONE Take a snapshot.
//      Capture a snapshot (cumulative sum) of the total stake and broken-down delegation balances.
//      SnapshotFrames capture worker-level aggregate stake balances.
//      SnapshotEntries capture delegation-level individual stake balances.

// DONE Cutover from current epoch to new epoch.

#[derive(Accounts)]
pub struct RegistryEpochKickoff<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.epoch_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        constraint = snapshot.id.eq(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<RegistryEpochKickoff>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let queue = &ctx.accounts.queue;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;

    // Lock the registry
    registry.locked = true;

    // Setup the next kickoff instruction to use the next snapshot.
    let kickoff_instruction = Some(InstructionData {
        program_id: crate::ID,
        accounts: vec![
            AccountMetaData::new_readonly(config.key(), false),
            AccountMetaData::new_readonly(queue.key(), true),
            AccountMetaData::new(registry.key(), false),
            AccountMetaData::new_readonly(
                Snapshot::pubkey(snapshot.id.checked_add(1).unwrap()),
                false,
            ),
        ],
        data: anchor_sighash("registry_epoch_kickoff").to_vec(),
    });

    // Build the next instruction for queue.
    let next_instruction = if snapshot.total_frames.gt(&0) {
        // The current snapshot has frames. Distribute fees collected by workers.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(Fee::pubkey(Worker::pubkey(0)), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new_readonly(SnapshotFrame::pubkey(snapshot.key(), 0), false),
                AccountMetaData::new(Worker::pubkey(0), false),
            ],
            data: anchor_sighash("worker_fees_distribute").to_vec(),
        })
    } else if registry.total_workers.gt(&0) {
        // The registry has workers. Begin delegating stakes to workers.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(Worker::pubkey(0), false),
            ],
            data: anchor_sighash("worker_delegations_stake").to_vec(),
        })
    } else {
        // Cutover to the next epoch.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new(registry.key(), false),
            ],
            data: anchor_sighash("registry_epoch_cutover").to_vec(),
        })
    };

    Ok(CrankResponse {
        kickoff_instruction,
        next_instruction,
    })
}
