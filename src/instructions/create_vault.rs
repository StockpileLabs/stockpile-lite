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

use crate::state::Vault;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateVaultArgs {
    pub vault: Vault,
}

/*
** Initializes a vault as a proxy account **
Recommended to use a Squads multisig as the main "vault_account" field.
*/
pub fn create_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateVaultArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let vault_account = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    assert!(
        payer.is_signer, 
        "Payer must be the signer."
    );

    let rent_minimum = (Rent::get()?).minimum_balance(Vault::SPACE);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            vault_account.key,
            rent_minimum,
            Vault::SPACE as u64,
            program_id,
        ),
        &[
            payer.clone(),
            vault_account.clone(),
            system_program.clone(),
        ],
        &[&[
            Vault::SEED_PREFIX.as_bytes(),
            payer.key.as_ref(),
            &[args.vault.bump],
        ]],
    )?;

    args.vault.serialize(&mut &mut vault_account.data.borrow_mut()[..])?;

    Ok(())
}