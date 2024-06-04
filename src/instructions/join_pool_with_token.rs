use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program::invoke_signed,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    program_error::ProgramError,
    program_pack::Pack
};

use spl_token::state::Account;

use crate::state::{AcceptanceStatus, Participant, Pool, PoolAccess};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct JoinPoolWithTokenArgs {
    pub participant: Participant,
}

/*
** Joins a pool & creates a participant account **
Must be called if pool entry is token gated. Requires ownership of 
a specific token, and minimum balance threshold to be met.
*/
pub fn join_pool_with_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: JoinPoolWithTokenArgs
) -> ProgramResult {
    let mut participant_info = Participant::new(
        args.participant.pool_id, 
        args.participant.vault_id, 
        args.participant.bump
    );
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter).unwrap();
    let pool_account = next_account_info(accounts_iter).unwrap();
    // let token_mint = next_account_info(accounts_iter).unwrap();
    let token_account = next_account_info(accounts_iter).unwrap();
    let payer = next_account_info(accounts_iter).unwrap();
    let system_program = next_account_info(accounts_iter).unwrap();

    let mut pool = Pool::try_from_slice(&pool_account.try_borrow_mut_data()?)?;

    let rent_minimum = (Rent::get()?).minimum_balance(Participant::SPACE);
    
    pool.is_active().unwrap();

    match &mut pool.pool_access {
        PoolAccess::Open => return Err(ProgramError::BorshIoError("Call join_pool to join.".to_string())),
        PoolAccess::Manual => return Err(ProgramError::BorshIoError("Call join_pool to request access".to_string())),
        PoolAccess::TokenGated(info) => {
            // Unpack token account data from bytes
            let token_account_info = token_account.data.borrow();
            let ata = Account::unpack(&token_account_info).unwrap();

            // Validate that the payer owns the token account
            assert_eq!(
                token_account.owner,
                payer.key,
                "Token Account must be owned by the payer"
            );

            // Validate that the ATA mint is correct
            assert_eq!(
                ata.mint,
                info.mint,
                "Token mint must match mint required by pool entry."
            );

            // Validate that the minimum balance is satisfied
            assert!(
                ata.amount >= info.minimum_balance,
                "Must be holding minimum balance required by pool entry."
            );

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
                    payer.key.as_ref(),
                    &[participant_info.bump],
                ]],
            )?;

            // Set status to accepted
            participant_info.status = AcceptanceStatus::Accepted;
            participant_info.serialize(&mut &mut participant_account.data.borrow_mut()[..])?;

            pool.participant_index += 1;
            pool.serialize(&mut &mut pool_account.data.borrow_mut()[..])?
        }
    };

    Ok(())
}