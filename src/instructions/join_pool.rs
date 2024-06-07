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
use solana_program::msg;

use crate::state::{
    AcceptanceStatus, 
    Participant, 
    Pool, 
    PoolAccess
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JoinPoolArgs {
    pub participant: Participant,
}

/*
** Joins a pool & creates a participant account **
Handles for "Open" and "Manual" approval options.
*/
pub fn join_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: JoinPoolArgs,
) -> ProgramResult {
    let mut participant_info = Participant::new(
        args.participant.pool_id, 
        args.participant.vault_id, 
        args.participant.bump
    );
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert!(
        payer.is_signer, 
        "Payer must be the signer."
    );

    let mut pool = Pool::try_from_slice(&pool_account.try_borrow_mut_data()?)?;

    let rent_minimum = (Rent::get()?).minimum_balance(Participant::SPACE);
    
    pool.is_active().unwrap();

    match pool.pool_access {
        PoolAccess::Open => {
            // Create participant account
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
                    pool_account.key.as_ref(),
                    &pool.participant_index.to_le_bytes(),
                    &[participant_info.bump],
                ]],
            )?;

            // Set status to accepted
            participant_info.status = AcceptanceStatus::Accepted;
            participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

            // Add participant
            pool.participant_index += 1;
            pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?
        },
        PoolAccess::Manual => {
            // Create participant account
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
                    pool_account.key.as_ref(),
                    &pool.participant_index.to_le_bytes(),
                    &[participant_info.bump],
                ]],
            )?;

            participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

            // Notify of pending approval. Pool admin will need to call "accept_participant"
            msg!("Approval pending to join this pool.");
        },
        PoolAccess::TokenGated(_info) => return Err(ProgramError::BorshIoError("Pool requires holding a specific token.".to_string()))
    };

    Ok(())
}