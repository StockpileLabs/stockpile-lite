use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    state::Pool, 
    utils::validate_is_signer
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreatePoolArgs {
    pub pool: Pool,
}

/*
**Initializes a QF pool**
Created with zero balance, so "fund_pool" will need to be called.
*/
pub fn create_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreatePoolArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let pool_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    validate_is_signer(payer)?;

    let rent_minimum = (Rent::get()?).minimum_balance(Pool::SPACE);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            pool_account.key,
            rent_minimum,
            Pool::SPACE as u64,
            program_id,
        ),
        &[
            payer.clone(),
            pool_account.clone(),
            system_program.clone(),
        ],
        &[&[
            Pool::SEED_PREFIX.as_bytes(),
            payer.key.as_ref(),
            &[args.pool.bump],
        ]],
    )?;

    args.pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?;

    Ok(())
}