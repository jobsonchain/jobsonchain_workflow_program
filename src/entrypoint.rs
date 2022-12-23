#![cfg(not(feature = "no-entrypoint"))]
use crate::processor::Processor;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the program i.e current program
    accounts: &[AccountInfo], // account informations
    instruction_data: &[u8], // Instruction data
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)?;

    Ok(())
}
