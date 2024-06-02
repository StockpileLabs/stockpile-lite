use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{clock::Clock, pubkey::Pubkey};

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Pool {
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub admins: Vec<Pubkey>,
    pub funders: Vec<Pubkey>,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const MAX_NAME_LEN: usize = 64;

    pub const SPACE: usize = 8  // Discriminator
    + 32                        // Pubkey
    + 32                        // Pubkey
    + 8                         // u64
    + 4                         // u32
    + 4;                        // u32

    pub fn new(
        name: String, 
        start: u64, 
        end: u64, 
        admins: Vec<Pubkey>, 
        access: PoolAccess, 
        bump: u8
    ) -> Self {
        Self {
            name,
            start,
            end,
            admins,
            funders: vec![],
            pool_state: PoolState::PendingStart,
            pool_access: access,
            bump,
        }
    }

    pub fn is_active(&mut self) -> Result<(), StockpileError> {
        let current_time = Clock::get()?.unix_timestamp as u64;
        if current_time > self.end {
            return Err(StockpileError::DefaultError.into());
        }
        if current_time > self.start {
            self.pool_state = PoolState::Active;
        }
        match self.pool_state {
            PoolState::PendingStart => Err(StockpileError::DefaultError.into()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(StockpileError::DefaultError.into()),
            PoolState::Closed => Err(StockpileError::DefaultError.into()),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum PoolState {
    PendingStart,
    Active,
    Distributed,
}

impl Default for PoolState {
    fn default() -> Self {
        PoolState::PendingStart
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub enum PoolAccess {
    Open,
    Manual,
    TokenGated,
}

impl Default for PoolAccess {
    fn default() -> Self {
        PoolAccess::Manual
    }
}