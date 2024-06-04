use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::error::StockpileError;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Vault {
    pub name: String,
    pub authority: Pubkey,
    pub vault_state: VaultState,
    pub bump: u8,
}

impl Vault {
    pub const SEED_PREFIX: &'static str = "vault";
    pub const MAX_NAME_LEN: usize = 64;

    pub const SPACE: usize = 64  // Name
    + 8                         // u64
    + 4                         // Enum (Singleton)
    + 1                         // u8
    + 64;                       // Padding

    pub fn new(
        name: String,
        authority: Pubkey,
        bump: u8
    ) -> Result<Self, StockpileError> {
        if name.as_bytes().len() > Self::MAX_NAME_LEN {
            return Err(StockpileError::DefaultError.into());
        }

        Ok(Self {
            name,
            authority,
            vault_state: VaultState::Active,
            bump,
        })
    }

    pub fn is_active(&mut self) -> Result<(), StockpileError> {
        match self.vault_state {
            VaultState::Closed => Err(StockpileError::DefaultError.into()),
            VaultState::Active => Ok(()),
            VaultState::Deactivated => Err(StockpileError::DefaultError.into()),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub enum VaultState {
    Active,
    Deactivated,
    Closed,
}

impl Default for VaultState {
    fn default() -> Self {
        VaultState::Active
    }
}