use solana_program::{
    program_error::ProgramError,
    pubkey::Pubkey
};

use crate::state::{AcceptanceStatus, Participant};

// Validates a participant is bound to a pool & accepted
pub fn validate_participant(
    participant: &Participant,
    pool_account: &Pubkey
) -> Result<(), ProgramError> {
    assert_eq!(
        participant.pool_id,
        *pool_account, 
        "Participant must bound to current pool."
    );
    assert_eq!(
        participant.status,
        AcceptanceStatus::Accepted,
        "Participant must have a valid acceptance."
    );
    Ok(())
}