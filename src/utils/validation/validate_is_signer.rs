use solana_program::{
    account_info::AccountInfo,
    program_error::ProgramError
};

// Validates an account is a signer
pub fn validate_is_signer(
    account: &AccountInfo
) -> Result<(), ProgramError> {
    assert!(
        account.is_signer, 
        "Selected account must be a signer."
    );
    Ok(())
}