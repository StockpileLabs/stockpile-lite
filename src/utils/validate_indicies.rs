use solana_program::program_error::ProgramError;

// Validates two provided indicies match
pub fn validate_indicies(
    index_one: u8,
    index_two: u8,
) -> Result<(), ProgramError> {
    assert_eq!(
        index_one,
        index_two,
        "Provided indicies are mismatched"
    );
    Ok(())
}