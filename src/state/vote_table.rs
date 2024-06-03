use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct VoteTable {
    pub pool: Pubkey,
    pub participant: Pubkey,
    pub table: BTreeMap<u8, VoteTicket>,
    pub index: u8,
    pub bump: u8,
}

impl VoteTable {
    pub const SEED_PREFIX: &'static str = "vote";
    pub const MAX_TABLE_SIZE: usize = 128;

    pub const SPACE: usize = 32 // Pubkey
    + 32                        // Pubkey
    + 1                         // u8
    + 1                         // u8
    + ((32 + 8 + 8 + 1) * 128); // BTreeMap (max 255 entries)

    pub fn new(
        pool: Pubkey, 
        participant: Pubkey, 
        table_index: u8, 
        bump: u8
    ) -> Result<Self, StockpileError> {
        Ok(Self {
            pool,
            participant,
            table: BTreeMap::new(),
            index: table_index,
            bump
        })
    }

    pub fn is_full(self) -> Result<bool, StockpileError> {
        if self.table.len() == Self::MAX_TABLE_SIZE {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn add_entry(&mut self, entry: VoteTicket) -> () {
        let length = self.table.len() as u8;
        self.table.insert(length + 1, entry);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum VoteStatus {
    Valid,
    Invalid
}

impl Default for VoteStatus {
    fn default() -> Self {
        VoteStatus::Valid
    }
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct VoteTicket {
    pub voter: Pubkey,
    pub amount: u64,
    pub timestamp: u64,
    pub vote_status: VoteStatus,
}