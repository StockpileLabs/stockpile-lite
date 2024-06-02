pub mod instructions;
pub mod processor;
pub mod state;
pub mod error;

use {crate::processor::process_instruction, solana_program::{entrypoint, declare_id}};

declare_id!("BNsqbpyoGuh66NJh5tRw6DNqy6y6X9s6LgF6yunDGRKt");

entrypoint!(process_instruction);