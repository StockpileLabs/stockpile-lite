pub mod create_pool;
pub mod create_vault;
pub mod join_pool;
pub mod refresh;
pub mod join_pool_with_token;
pub mod contribute_with_vote;
pub mod accept_participant;


pub use create_pool::*;
pub use create_vault::*;
pub use join_pool::*;
pub use refresh::*;
pub use join_pool_with_token::*;
pub use contribute_with_vote::*;
pub use accept_participant::*;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(BorshSerialize, BorshDeserialize, Debug, ShankInstruction)]
pub enum StockpileLite {
    #[account(0, writable, name="pool_account", desc="Master account for a QF round")]
    #[account(1, writable, signer, name="payer", desc = "Fee payer/Initializer")]
    #[account(2, name="system_program", desc = "System Program")]
    CreatePool(CreatePoolArgs),

    #[account(0, writable, name="vault_account", desc="Master account for a vault. This is a proxy for a beneficiary")]
    #[account(1, writable, signer, name="payer", desc = "Fee payer/Initializer")]
    #[account(2, name="system_program", desc = "System Program")]
    CreateVault(CreateVaultArgs),

    #[account(0, writable, name="participant_account", desc="Account denoting a participant, tied to both a pool and a vault")]
    #[account(1, writable, name="pool_account", desc="Master account for a QF round")]
    #[account(2, writable, signer, name="payer", desc = "Fee payer/Initializer")]
    #[account(3, name="system_program", desc = "System Program")]
    JoinPool(JoinPoolArgs),

    Refresh(RefreshArgs),

    #[account(0, writable, name="participant_account", desc="Account denoting a participant, tied to both a pool and a vault")]
    #[account(1, writable, name="pool_account", desc="Master account for a QF round")]
    #[account(2, writable, name="vote_table_account", desc="Current vote table for the participant")]
    #[account(3, name="mint", desc="Mint account for the pool")]
    #[account(4, writable, name="target_vault", desc="Master account for the target vault")]
    #[account(5, writable, name="vault_authority_token_account", desc="Token account for the vault's authority key")]
    #[account(6, writable, signer, name="payer", desc="Fee payer")]
    #[account(7, writable, name="payer_token_account", desc="Fee payer's token account")]
    #[account(8, name="system_program", desc="System Program")]
    #[account(9, name="token_program", desc="Token Program")]
    #[account(10, writable, optional, name="next_vote_table_account", desc="Derived key for the next vote table, if the current one is full.")]
    #[account(11, signer, optional, name="relayer_key", desc="Relayer key for 'Relayer' sybil strategy, if specified")]
    ContributeWithVote(ContributeWithVoteArgs),

    #[account(0, writable, name="participant_account", desc="Account denoting a participant that's being accepted")]
    #[account(1, writable, name="pool_account", desc="Master account for a QF round")]
    #[account(2, writable, name="vote_table_account", desc="Current vote table for the participant")]
    #[account(3, writable, signer, name="payer", desc = "Fee payer/Initializer")]
    #[account(4, name="system_program", desc = "System Program")]
    AcceptParticipant()
}