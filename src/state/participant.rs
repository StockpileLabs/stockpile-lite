use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{sysvar::Sysvar, clock::Clock};
use solana_sdk::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Default)]
pub struct Participant {
    pub pool_id: Pubkey,
    pub vault_id: Pubkey,
    pub timestamp: u64,
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
            status: AcceptanceStatus::Pending,
            bump
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub enum AcceptanceStatus {
    Accepted,
    Pending,
    Denied,
}

impl Default for AcceptanceStatus {
    fn default() -> Self {
        AcceptanceStatus::Pending
    }
}