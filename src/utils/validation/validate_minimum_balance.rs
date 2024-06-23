use solana_program::program_error::ProgramError;
use spl_token::state::Account;

use crate::state::TokenGateInfo;

// Validates a token account holds minimum pool balance
pub fn validate_minimum_balance(
    info: &mut TokenGateInfo,
    ata: Account
) -> Result<(), ProgramError> {
    assert!(
        ata.amount >= info.minimum_balance,
        "Must be holding minimum balance required by pool entry."
    );
    Ok(())
}