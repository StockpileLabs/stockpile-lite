use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{sysvar::Sysvar, clock::Clock};
use solana_program::pubkey::Pubkey;

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct Pool {
    pub name: String,
    pub start: u64,
    pub end: u64,
    pub mint: Pubkey,
    pub admins: Vec<Pubkey>,
    // Vector of keys that point to Participant accounts
    pub participants: Vec<Pubkey>,
    pub pool_state: PoolState,
    pub pool_access: PoolAccess,
    pub sybil_strategy: SybilStrategy,
    pub table_index: u8,
    pub bump: u8,
}

impl Pool {
    pub const SEED_PREFIX: &'static str = "pool";
    pub const MAX_NAME_LEN: usize = 64;
    pub const MAX_ADMINS: usize = 5;
    pub const MAX_PARTICIPANTS: usize = 32;

    pub const SPACE: usize = Self::MAX_NAME_LEN  // Name
    + 8                                          // u64
    + 8                                          // u64
    + (32 * Self::MAX_ADMINS)                    // Vec<Pubkey>: Max 5
    + (32 * Self::MAX_PARTICIPANTS)              // Vec<Pubkey>: Max 32
    + 1                                          // Enum (Singleton)
    + 4                                          // Enum
    + 1                                          // Enum
    + 1                                          // u8
    + 1                                          // u8
    + 128;                                       // Padding

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
            mint,
            admins,
            participants: vec![],
            pool_state: PoolState::PendingStart,
            pool_access: access,
            sybil_strategy: sybil,
            table_index: 0,
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
    TokenGated(TokenGateInfo),
}

impl Default for PoolAccess {
    fn default() -> Self {
        PoolAccess::Manual
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub struct TokenGateInfo {
    pub mint: Pubkey,
    pub minimum_balance: u64
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum SybilStrategy {
    None,
    Civic
}

impl Default for SybilStrategy {
    fn default() -> Self {
        SybilStrategy::None
    }
}