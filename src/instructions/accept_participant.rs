use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::invoke_signed,
    pubkey::Pubkey,
    program_error::ProgramError,
    system_instruction,
    rent::Rent,
    sysvar::Sysvar
};

use crate::state::{
    AcceptanceStatus, 
    Participant, 
    Pool, 
    PoolAccess, 
    VoteTable
};

/*
** Accepts a participant and creates a vote table for them **
*/
pub fn accept_participant(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter).unwrap();
    let pool_account = next_account_info(accounts_iter)?;
    let vote_table_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let mut pool = Pool::try_from_slice(&pool_account.try_borrow_mut_data()?)?;
    let mut participant_info = Participant::try_from_slice(&participant_account.try_borrow_mut_data()?)?;

    let rent_minimum = (Rent::get()?).minimum_balance(VoteTable::SPACE);

    pool.is_active().unwrap();

    match pool.pool_access {
        PoolAccess::Open => return Err(ProgramError::BorshIoError("Pool is open. Call join_pool to participate.".to_string())),
        PoolAccess::Manual => {
           // Check that participant account is tied to current pool
           assert_eq!(
              participant_info.pool_id,
              *pool_account.key,
              "Participant account must be tied to current pool."
           );

           // Accept participant
           participant_info.status = AcceptanceStatus::Accepted;
           participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

           // Derive vote table PDA
           let (_table_pda, bump) = Pubkey::find_program_address(
            &[
                    VoteTable::SEED_PREFIX.as_bytes(),
                    participant_account.key.as_ref(),
                    &participant_info.table_index.to_le_bytes()
                ], 
                program_id
            );

            // Initialize first vote table
            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    vote_table_account.key,
                    rent_minimum,
                    VoteTable::SPACE as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    vote_table_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    VoteTable::SEED_PREFIX.as_bytes(),
                    participant_account.key.as_ref(),
                    &participant_info.table_index.to_le_bytes(),
                    &[bump],
                ]],
            )?;

            // Create new instance of vote table, serialize to new account
            let table = VoteTable::new(
                *pool_account.key, 
                *participant_account.key, 
                participant_info.table_index, 
                bump
            ).unwrap();

            table.serialize(&mut &mut vote_table_account.data.borrow_mut()[..])?;
        },
        PoolAccess::TokenGated(_info) => return Err(ProgramError::BorshIoError("Pool is open to holders of a specific token.".to_string()))
    };

    Ok(())
}