use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{sysvar::Sysvar, clock::Clock};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Default, ShankAccount)]
pub struct Participant {
    pub pool_id: Pubkey,
    pub vault_id: Pubkey,
    pub timestamp: u64,
    pub table_index: u8,
    pub status: AcceptanceStatus,
    pub bump: u8
}

impl Participant {
    pub const SEED_PREFIX: &'static str = "participant";
    pub const SPACE: usize = 64  // Name
    + 8                         // u64
    + 8                         // u64
    + 32                        // Vec<Pubkey>: Max 5
    + 32                        // Vec<Pubkey>
    + 8                         // u64
    + 4                         // u32
    + 4;                        // u32

    pub fn new(pool_id: Pubkey, vault_id: Pubkey, bump: u8) -> Self {
        // TODO: add validations if needed (should probably check pool acct infos)
        let current = Clock::get().unwrap();
        let time = current.unix_timestamp as u64;
        
        Self {
            pool_id,
            vault_id,
            timestamp: time,
            table_index: 0,
            status: AcceptanceStatus::Pending,
            bump
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug, Default)]
pub enum AcceptanceStatus {
    Accepted,
    #[default]
    Pending,
    Denied,
}