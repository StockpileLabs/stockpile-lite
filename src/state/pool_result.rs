use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey
};

/// Stores pool result data. This account will continuously be written to in the process
/// of calculation results, which will likely happen over the course of multiple transactions.
/// These will not be happening atomically, so proper state handling is important.
#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, ShankAccount)]
pub struct PoolResult {
    pub pool_id: Pubkey,    /// Pool in question
    pub s_sq_all: f64,      /// Total Sum of Square Roots
    pub sq_sums: BTreeMap<Pubkey, f64>,     /// Sum of Square Roots per participant
    pub final_proportions: Option<BTreeMap<Pubkey, f64>>,   /// Final calculated proportions
    pub progress: ResultState,      /// State of the result calculation process
    pub bump: u8,
}

impl PoolResult {
    pub const SEED_PREFIX: &'static str = "result";

    pub fn new(
        pool_id: Pubkey,
        s_sq_all: f64,
        bump: u8,
    ) -> Result<Self, ProgramError> {
        Ok(Self {
            pool_id,
            s_sq_all,
            sq_sums: BTreeMap::new(),
            final_proportions: Some(BTreeMap::new()),
            progress: ResultState::NotStarted,
            bump
        })
    }

    pub fn get_space(
        participant_length: usize
    ) -> usize {
        let size: usize = 32                         // Name
        + 8                                          // f64
        + 8                                          // u64
        + ((32 * 8) * participant_length)            // BTreeMap<Pubkey, f64>: Max 255
        + ((32 * 8) * participant_length)            // Option<BTreeMap<Pubkey, f64>>: Max 255
        + 4                                          // Enum (Singleton)
        + 1                                          // u8
        + 128;                                       // Padding
        
        return size
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub enum ResultState {
    NotStarted,             // Calculations have not begun yet
    Pending,                // Calculations are in progress, but not all participants & tables have been accounted
    Confirmed,              // Participant calculations are confirmed, final proportions are pending
    Completed               // All calculations completed
}

impl Default for ResultState {
    fn default() -> Self {
        ResultState::NotStarted
    }
}