use borsh::BorshDeserialize;
use initialize_config::process_initialize_config;
use initialize_operator_history_account::process_initialize_operator_history_account;
use operator_history_sdk::instruction::OperatorHistoryInstruction;
use realloc_operator_history_account::process_realloc_operator_history_account;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;

mod initialize_config;
mod initialize_operator_history_account;
mod realloc_operator_history_account;

solana_pubkey::declare_id!(env!("OPERATOR_HISTORY_PROGRAM_ID"));

#[cfg(not(feature = "no-entrypoint"))]
solana_program_entrypoint::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = OperatorHistoryInstruction::try_from_slice(instruction_data)?;

    match instruction {
        OperatorHistoryInstruction::InitializeConfig => {
            msg!("Instruction: InitializeConfig");
            process_initialize_config(program_id, accounts)
        }
        OperatorHistoryInstruction::InitializeOperatorHistoryAccount => {
            msg!("Instruction: InitializeOperatorHistoryAccount");
            process_initialize_operator_history_account(program_id, accounts)
        }
        OperatorHistoryInstruction::ReallocOperatorHistoryAccount => {
            msg!("Instruction: ReallocOperatorHistoryAccount");
            process_realloc_operator_history_account(program_id, accounts)
        }
    }
}
