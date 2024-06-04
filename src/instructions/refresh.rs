use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::state::{Participant, Pool};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct RefreshArgs {
    pub pool: Pool,
    pub participants: Vec<Participant>,
}

/* 
** Refreshes vote data for a given pool **
This is only recommended to be used with smaller pools, as compute bounds
will be hit given enough participation in a pool. If conducting a round with
a larger pool, consider deriving the necessary accounts and calculating pool 
shares via the client. This will still be verifiable on-chain.
*/
pub fn refresh(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: RefreshArgs
) -> ProgramResult {

    Ok(())
}