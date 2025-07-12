use borsh::BorshSerialize;
use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use crate::instruction::OperatorHistoryInstruction;

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    admin: &Pubkey,
    vault_program: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new(*admin, true),
        AccountMeta::new_readonly(*vault_program, false),
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
