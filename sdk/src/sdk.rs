use borsh::BorshSerialize;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use crate::instruction::OperatorHistoryInstruction;

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    admin: &Pubkey,
    jito_restaking_program: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(*jito_restaking_program, false),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: OperatorHistoryInstruction::InitializeConfig
            .try_to_vec()
            .unwrap(),
    }
}

pub fn initialize_operator_history_account(
    program_id: &Pubkey,
    config: &Pubkey,
    operator_history: &Pubkey,
    operator: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*config, false),
        AccountMeta::new(*operator_history, false),
        AccountMeta::new_readonly(*operator, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: OperatorHistoryInstruction::InitializeOperatorHistoryAccount
            .try_to_vec()
            .unwrap(),
    }
}

pub fn realloc_operator_history_account(
    program_id: &Pubkey,
    config: &Pubkey,
    operator_history: &Pubkey,
    operator: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*config, false),
        AccountMeta::new(*operator_history, false),
        AccountMeta::new_readonly(*operator, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: OperatorHistoryInstruction::ReallocOperatorHistoryAccount
            .try_to_vec()
            .unwrap(),
    }
}
