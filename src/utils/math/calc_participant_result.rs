use solana_program::program_error::ProgramError;

// Calculates the final square, given chunks from each vote_table
pub fn calc_participant_result(
    chunks: Vec<f64>
) -> Result<f64, ProgramError> {
    let sum: f64 = chunks.iter().sum();
    Ok(sum.powi(2))
}