use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    program_pack::Pack,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    msg
};

use spl_token::{state::Account, instruction::transfer};

use crate::state::{
    AcceptanceStatus, 
    Participant, 
    Pool, 
    SybilStrategy, 
    Vault, 
    VoteTable, 
    VoteTicket
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ContributeWithVoteArgs {
    pub amount: u64,
}

/* 
** Contributes to a vault and casts a vote **
This code fucking sucks and I'll probably be refactoring later
*/
pub fn contribute_with_vote(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ContributeWithVoteArgs
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let vote_table_account = next_account_info(accounts_iter)?;
    let next_vote_table_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    // This account will not recieve the tokens directly, only its authority
    let target_vault = next_account_info(accounts_iter)?;
    let vault_authority_token_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let payer_token_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Create a vote ticket from args
    let ticket = VoteTicket::new(
        *payer.key,
        args.amount,
    ).unwrap();

    // Deserialize accounts
    let mut pool = Pool::try_from_slice(&pool_account.try_borrow_mut_data()?)?;
    let mut participant = Participant::try_from_slice(&participant_account.try_borrow_mut_data()?)?;
    let mut vote_table = VoteTable::try_from_slice(&vote_table_account.try_borrow_mut_data()?)?;

    // Deserialize payer ATA
    let token_account_info = payer_token_account.data.borrow();
    let ata = Account::unpack(&token_account_info)?;

    // Get rent minimum for a new vote table
    let rent_minimum = (Rent::get()?).minimum_balance(VoteTable::SPACE);

    // TODO: Sybil check should happen here, do a match against sybil strategy
    match pool.sybil_strategy {
        SybilStrategy::None => msg!("No sybil strategy, skipping checks..."),
        SybilStrategy::Civic => msg!("Civic strategy...")
    }
    // ^^ Actually do something here besides log and eat hot chip

    // Validation checks
    assert_eq!(
        participant.pool_id,
        *pool_account.key, 
        "Participant must bound to current pool."
    );

    assert_eq!(
        participant.status,
        AcceptanceStatus::Accepted,
        "Participant must have a valid acceptance."
    );

    assert_eq!(
        ata.mint,
        pool.mint,
        "Token mint must match default pool mint."
    );

    pool.is_active().unwrap();

    // Check if full, respond accordingly
    match vote_table.clone().is_full().unwrap() {
        true => {
            // Increment index
            participant.table_index += 1;

            let (_table_pda, bump) = Pubkey::find_program_address(
                &[
                    VoteTable::SEED_PREFIX.as_bytes(),
                    participant_account.key.as_ref(),
                    &participant.table_index.to_le_bytes()
                ], 
                program_id
            );

            // Initialize new vote table account
            invoke_signed(
                &system_instruction::create_account(
                    payer.key,
                    next_vote_table_account.key,
                    rent_minimum,
                    VoteTable::SPACE as u64,
                    program_id,
                ),
                &[
                    payer.clone(),
                    next_vote_table_account.clone(),
                    system_program.clone(),
                ],
                &[&[
                    VoteTable::SEED_PREFIX.as_bytes(),
                    participant_account.key.as_ref(),
                    &participant.table_index.to_le_bytes(),
                    &[bump],
                ]],
            )?;

            // Create new instance of vote table
            let new_table = VoteTable::new_with_voter(
                *pool_account.key, 
                *participant_account.key, 
                ticket, 
                participant.table_index, 
                bump
            ).unwrap();

            new_table.serialize(&mut &mut next_vote_table_account.data.borrow_mut()[..])?;
        }
        false => {
            // Derived PDA w/ current index
            let (table_pda, _bump) = Pubkey::find_program_address(
                &[
                    VoteTable::SEED_PREFIX.as_bytes(),
                    participant_account.key.as_ref(),
                    &participant.table_index.to_le_bytes()
                ], 
                program_id
            );
        
            // Validate account matches the one passed in
            assert_eq!(
                table_pda,
                *vote_table_account.key,
                "Vote table account must be derived from the current index"
            );

            // Add entry to table
            vote_table.add_entry(ticket).unwrap();
        }
    }
    
    vote_table.serialize(&mut &mut vote_table_account.data.borrow_mut()[..])?;

    // Deserialize recipient ATA
    let target_token_account_info = vault_authority_token_account.data.borrow();
    let target_ata = Account::unpack(&target_token_account_info)?;

    // Deserialize vault account
    let target_vault_inner = Vault::try_from_slice(&target_vault.try_borrow_mut_data()?)?;

    // Validate the ATA is owned by the address the vault is a proxy
    // for, not the vault itself
    assert_eq!(
        target_ata.owner,
        target_vault_inner.authority,
        "Token account must be owned by the target vault"
    );

    // Transfer corresponding tokens
    invoke(
        &transfer(
            token_program.key,
            payer_token_account.key,
            vault_authority_token_account.key,
            payer.key,
            &[payer.key],
            args.amount,
        )?,
        &[
            mint.clone(),
            payer_token_account.clone(),
            vault_authority_token_account.clone(),
            payer.clone(),
            //recipient.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}