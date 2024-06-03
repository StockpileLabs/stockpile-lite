use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{sysvar::Sysvar, clock::Clock};
use solana_sdk::pubkey::Pubkey;

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Pool {
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub admins: Vec<Pubkey>,
    // Vector of keys that point to Participant accounts
    pub participants: Vec<Pubkey>,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const MAX_NAME_LEN: usize = 64;

    pub const SPACE: usize = 64  // Name
    + 8                         // u64
    + 8                         // u64
    + 32                        // Vec<Pubkey>: Max 5
    + 32                        // Vec<Pubkey>
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
    ) -> Result<Self, StockpileError> {
        if name.as_bytes().len() > Self::MAX_NAME_LEN {
            return Err(StockpileError::DefaultError.into());
        }

        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        if timestamp > start {
            return Err(StockpileError::DefaultError.into());
        }

        Ok(Self {
            name,
            start,
            end,
            admins,
            participants: vec![],
            pool_state: PoolState::PendingStart,
            pool_access: access,
            bump,
        })
    }

    pub fn is_active(&mut self) -> Result<(), StockpileError> {
        let current = Clock::get().unwrap();
        let timestamp = current.unix_timestamp as u64;
        if timestamp > self.end {
            return Err(StockpileError::DefaultError.into());
        }

        if timestamp > self.start {
            self.pool_state = PoolState::Active;
        }

        match self.pool_state {
            PoolState::PendingStart => Err(StockpileError::DefaultError.into()),
            PoolState::Active => Ok(()),
            PoolState::Distributed => Err(StockpileError::DefaultError.into()),
        }
    }

    pub fn add_participant(&mut self, participant_account: Pubkey) -> Result<(), StockpileError> {
        self.participants.push(
            participant_account
        );

        Ok(())
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