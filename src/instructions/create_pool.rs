use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::state::Pool;

pub fn create_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool: Pool,
) -> ProgramResult {
    Ok(())
}