use std::collections::BTreeMap;

use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey
};

// Calculates proportions given all 
pub fn calc_pool_result(
    vault_key: Pubkey,
    votes: BTreeMap<Pubkey, f64>,
    s_sq_all: f64,
) -> Result<f64, ProgramError> {
    let final_proportion = match votes.get(&vault_key) {
        Some(vote_count) => vote_count / s_sq_all,
        None => return Err(ProgramError::BorshIoError("Key was not found in vote data.".to_string()))
    };

    Ok(final_proportion)
}