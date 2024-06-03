use crate::instructions::*;
use crate::state::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StockpileLite {
    CreatePool(Pool),
    CreateVault(Vault),
    JoinPool(Participant),
    Refresh()
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StockpileLite::try_from_slice(input)?;
    match instruction {
        StockpileLite::CreatePool(data) => create_pool(program_id, accounts, CreatePoolArgs { pool: data }),
        StockpileLite::CreateVault(data) => create_vault(program_id, accounts, CreateVaultArgs { vault: data }),
        StockpileLite::JoinPool(data) => join_pool(program_id, accounts, JoinPoolArgs { participant: data }),
        StockpileLite::Refresh() => refresh(program_id, accounts)
    }
}