use solana_program::{
    pubkey::Pubkey,
    program_error::ProgramError
};
use spl_token::state::Account;

// Validates a token account's owner and mint are as expected
pub fn validate_ata(
    expected_owner: Pubkey,
    expected_mint: Pubkey,
    ata: Account
) -> Result<(), ProgramError> {
    assert_eq!(
        ata.owner,
        expected_owner,
        "Token Account must be owned by the expected account"
    );
    assert_eq!(
        ata.mint,
        expected_mint,
        "Token account mint must match expected mint"
    );
    Ok(())
}