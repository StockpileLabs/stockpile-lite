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
    program_error::ProgramError,
    instruction::Instruction,
    instruction::AccountMeta,
    msg
};

use spl_token::{
    state::Account, 
    instruction::transfer
};

use crate::{state::{
    Participant, 
    Pool, 
    SybilStrategy, 
    Vault, 
    VoteTable, 
    VoteTicket
}, utils::{
    validate_ata, 
    validate_indicies, 
    validate_is_signer, 
    validate_participant
}};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ContributeWithVoteArgs {
    pub amount: u64,
    pub next_bump: Option<u8>
}

/* 
** Contributes to a vault and casts a vote **
This code fucking sucks and I'll probably be refactoring later
*/
pub fn contribute_with_vote(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ContributeWithVoteArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let participant_account = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let vote_table_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    // This account will not recieve the tokens directly, only its authority
    let target_vault = next_account_info(accounts_iter)?;
    let vault_authority_token_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let payer_token_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    // Technically required even though we may not need it
    let next_vote_table_account = next_account_info(accounts_iter)?;
    // Optional
    let relayer_key = next_account_info(accounts_iter)?;

    validate_is_signer(payer)?;

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

    // Validation checks
    validate_participant(
        &participant, 
        pool_account.key
    )?;

    validate_ata(
        *payer.key, 
        pool.mint, 
        ata
    )?;

    pool.is_active().unwrap();

    // Get rent minimum for a new vote table
    let rent_minimum = (Rent::get()?).minimum_balance(VoteTable::SPACE);

    // Check if full, respond accordingly
    match vote_table.clone().is_full().unwrap() {
        true => {
            // Increment index
            participant.table_index += 1;

            // Unwrap bump passed in from client, default to 255 if none
            let new_acc_bump = args.next_bump.unwrap_or(255);

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
                    &[new_acc_bump],
                ]],
            )?;

            // Create new instance of vote table
            let new_table = VoteTable::new_with_voter(
                *pool_account.key, 
                *participant_account.key, 
                ticket, 
                participant.table_index, 
                new_acc_bump
            ).unwrap();

            new_table.serialize(&mut &mut next_vote_table_account.data.borrow_mut()[..])?;
        }
        false => {
            // Validate table account uses the correct index
            validate_indicies(
                participant.table_index, 
                vote_table.index
            )?;

            // Add entry to table
            vote_table.add_entry(ticket).unwrap();
        }
    }

    vote_table.serialize(&mut &mut vote_table_account.data.borrow_mut()[..])?;

    match pool.sybil_strategy {
        SybilStrategy::None => msg!("No sybil strategy, skipping checks..."),
        SybilStrategy::Relayer(signers) => {
            if relayer_key.is_signer && signers.contains(relayer_key.key) {
                // Mr. Gorbachev, tear down this wall.
            } else {
                return Err(ProgramError::BorshIoError("Relayer must sign if strategy is set to `relayer`".to_string()))
            }
        },
        SybilStrategy::Custom(strategy) => {
            // 11 accounts needed in base case if it hits this leg
            let remaining_accounts = &accounts[11..];

            let mut account_metas: Vec<AccountMeta> = remaining_accounts
                .iter()
                .map(|acc| AccountMeta::new(*acc.key, false))
                .collect();

            account_metas.extend([
                AccountMeta::new(*payer.key, true),
                AccountMeta::new(*system_program.key, false)
            ]);
            
            // Deserialize instruction
            let ix = Instruction::new_with_bytes(
                strategy.program_id, 
                &strategy.data, 
                account_metas
            );

            // Invoke the instruction
            invoke(
                &ix, 
                // Concat the current payer and their token account
                remaining_accounts
            )?;
        }
    }

    // Deserialize recipient ATA
    let target_token_account_info = vault_authority_token_account.data.borrow();
    let target_ata = Account::unpack(&target_token_account_info)?;

    // Deserialize vault account
    let target_vault_inner = Vault::try_from_slice(&target_vault.try_borrow_mut_data()?)?;

    // Validate the ATA is owned by the address the vault is a proxy
    // for, not the vault itself
    validate_ata(
        target_vault_inner.authority, 
        *mint.key, 
        target_ata
    )?;

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