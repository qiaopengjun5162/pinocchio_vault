#![no_std]
use pinocchio::{
    AccountView, Address, ProgramResult, entrypoint, error::ProgramError, nostd_panic_handler,
};
use solana_address::declare_id;
use solana_program_log::log;

nostd_panic_handler!();
entrypoint!(process_instruction);

pub mod instructions;
pub use instructions::*;

// 22222222222222222222222222222222222222222222
// pub const ID: Address = address!("22222222222222222222222222222222222222222222");
// declare_id!("22222222222222222222222222222222222222222222");
declare_id!("A11gcDm7e8Pit4RiunfhtrK1BKU4oYAa3nx54R4YnFgS");

fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    log("Hello from my pinocchio program!");

    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
