use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{sysvar::Sysvar, clock::Clock};
use solana_program::pubkey::Pubkey;

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, ShankAccount)]
pub struct Pool {
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub mint: Pubkey,
    pub admins: Vec<Pubkey>,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub sybil_strategy: SybilStrategy,
    pub participant_index: u8,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const MAX_NAME_LEN: usize = 64;
    pub const MAX_ADMINS: usize = 5;
    pub const MAX_PARTICIPANTS: usize = 255;

    pub const SPACE: usize = Self::MAX_NAME_LEN  // Name
    + 8                                          // u64
    + 8                                          // u64
    + (32 * Self::MAX_ADMINS)                    // Vec<Pubkey>: Max 5
    + 1                                          // Enum (Singleton)
    + 4                                          // Enum
    + 1                                          // Enum
    + 1                                          // u8
    + 1                                          // u8
    + 256;                                       // Padding (Custom Sybil Impl)

    pub fn new(
        name: String, 
        start: u64, 
        end: u64, 
        mint: Pubkey,
        admins: Vec<Pubkey>, 
        access: PoolAccess, 
        sybil: SybilStrategy,
        bump: u8
    ) -> Result<Self, StockpileError> {
        if name.as_bytes().len() > Self::MAX_NAME_LEN {
            return Err(StockpileError::DefaultError);
        }

        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        if timestamp > start {
            return Err(StockpileError::DefaultError);
        }

        Ok(Self {
            name,
            start,
            end,
            mint,
            admins,
            pool_state: PoolState::PendingStart,
            pool_access: access,
            sybil_strategy: sybil,
            participant_index: 0,
            bump,
        })
    }

    pub fn is_active(&mut self) -> Result<(), StockpileError> {
        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        if timestamp > self.end {
            return Err(StockpileError::DefaultError);
        }

        if timestamp > self.start {
            self.pool_state = PoolState::Active;
        }

        match self.pool_state {
            PoolState::PendingStart => Err(StockpileError::DefaultError),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(StockpileError::DefaultError),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Default)]
pub enum PoolState {
    #[default]
    PendingStart,
    Active,
    Distributed,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug, Default)]
pub enum PoolAccess {
    Open,
    #[default]
    Manual,
    TokenGated(TokenGateInfo),
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub struct TokenGateInfo {
    pub mint: Pubkey,
    pub minimum_balance: u64
}

// SYBIL STRATEGIES
// 1. None - Performs zero checks on a contributor, this is the default option
// 2. Civic - Runs a Civic Gateway check on the user & requires them to have a pass
// 3. Relayer - Requires signing by specified keys (usually via an API endpoint)
// 4. Custom - Add a serialized ix, and perform the check via your own on-chain program.
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, Default)]
pub enum SybilStrategy {
    #[default]
    None,
    Relayer(Vec<Pubkey>),
    Custom(CustomStrategy)
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CustomStrategy {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>
}