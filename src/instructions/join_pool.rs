use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    program_error::ProgramError
};
use solana_sdk::msg;

use crate::state::{AcceptanceStatus, Participant, Pool, PoolAccess};

/*
** Joins a pool & creates a participant account **
*/
pub fn join_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    participant: Participant,
) -> ProgramResult {
    let mut participant_info = Participant::new(
        participant.pool_id, 
        participant.vault_id, 
        participant.bump
    );
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter).unwrap();
    let pool_account = next_account_info(accounts_iter).unwrap();
    let payer = next_account_info(accounts_iter).unwrap();
    let system_program = next_account_info(accounts_iter).unwrap();

    let mut pool = Pool::try_from_slice(&pool_account.try_borrow_mut_data()?)?;

    let rent_minimum = (Rent::get()?).minimum_balance(Participant::SPACE);
    
    pool.is_active().unwrap();

    match pool.pool_access {
        PoolAccess::Open => {
            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    participant_account.key,
                    rent_minimum,
                    Participant::SPACE as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    participant_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    Participant::SEED_PREFIX.as_bytes(),
                    payer.key.as_ref(),
                    &[participant_info.bump],
                ]],
            )?;

            participant_info.status = AcceptanceStatus::Accepted;
            participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

            pool.add_participant(*participant_account.key).unwrap();
            pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?
        },
        // TODO: Create account and prompt approval from an admin
        PoolAccess::Manual => {
            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    participant_account.key,
                    rent_minimum,
                    Participant::SPACE as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    participant_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    Participant::SEED_PREFIX.as_bytes(),
                    payer.key.as_ref(),
                    &[participant_info.bump],
                ]],
            )?;

            participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

            msg!("Approval pending to join this pool.");
        },
        PoolAccess::TokenGated => return Err(ProgramError::BorshIoError("Pool requires holding a specific token.".to_string()))
    };

    Ok(())
}