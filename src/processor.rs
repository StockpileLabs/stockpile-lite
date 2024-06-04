use crate::instructions::*;
use crate::state::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum StockpileLite {
    CreatePool(CreatePoolArgs),
    CreateVault(CreateVaultArgs),
    JoinPool(JoinPoolArgs),
    Refresh(RefreshArgs),
    ContributeWithVote(ContributeWithVoteArgs),
    AcceptParticipant()
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = StockpileLite::try_from_slice(input)?;
    match instruction {
        StockpileLite::CreatePool(data) => create_pool(program_id, accounts, CreatePoolArgs { pool: data.pool }),
        StockpileLite::CreateVault(data) => create_vault(program_id, accounts, CreateVaultArgs { vault: data.vault }),
        StockpileLite::JoinPool(data) => join_pool(program_id, accounts, JoinPoolArgs { participant: data.participant }),
        StockpileLite::Refresh(data) => refresh(program_id, accounts, RefreshArgs { pool: data.pool, participants: data.participants }),
        StockpileLite::ContributeWithVote(data) => contribute_with_vote(program_id, accounts, ContributeWithVoteArgs { amount: data.amount }),
        StockpileLite::AcceptParticipant() => accept_participant(program_id, accounts)
    }
}