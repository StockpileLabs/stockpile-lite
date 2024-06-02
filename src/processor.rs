use crate::instructions::*;
use crate::state::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StockpileLite {
    CreatePool(Pool),
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StockpileLite::try_from_slice(input)?;
    match instruction {
        StockpileLite::CreatePool(data) => create_pool(program_id, accounts, data),
    }
}