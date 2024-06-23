use solana_program::program_error::ProgramError;

use crate::state::VoteTable;

// Calculates the sum of square roots for a single vote table account
pub fn calc_sum_sqrt(
    vote_table: VoteTable
) -> Result<f64, ProgramError> {
    let sum_of_square_roots: f64 = vote_table.table.values()
        .map(|vote| (vote.amount as f64).sqrt())
        .sum();

    Ok(sum_of_square_roots)
}