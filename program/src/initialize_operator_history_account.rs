use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_signer, load_system_account, load_system_program};
use jito_restaking_core::operator::Operator;
use operator_history_core::{config::Config, operator_history::OperatorHistory, MAX_REALLOC_BYTES};
use operator_history_sdk::error::OperatorHistoryError;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program::program::{invoke, invoke_signed};
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_sysvar::Sysvar;

/// Initializes the [`OperatorHistory`] account
pub fn process_initialize_operator_history_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [config_info, operator_history_info, operator_info, payer, system_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Config::load(program_id, config_info, false)?;
    let config_data = config_info.data.borrow();
    let config = Config::try_from_slice_unchecked(&config_data)?;

    Operator::load(&config.jito_restaking_program_id(), operator_info, false)?;

    load_system_account(operator_history_info, true)?;
    load_signer(payer, true)?;
    load_system_program(system_program)?;

    // The OperatorHistory shall be at the canonical PDA
    let (operator_history_pubkey, operator_history_bump, mut operator_history_seeds) =
        OperatorHistory::find_program_address(program_id, operator_info.key);
    operator_history_seeds.push(vec![operator_history_bump]);
    if operator_history_info.key.ne(&operator_history_pubkey) {
        msg!("OperatorHistory account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Initializing OperatorHistory at address {operator_history_pubkey}");
    let rent = &Rent::get()?;
    let data_len = 8_u64
        .checked_add(size_of::<OperatorHistory>() as u64)
        .ok_or(OperatorHistoryError::Arithmetic)?;
    let minimum_balance = rent.minimum_balance(data_len as usize);
    let required_lamports = minimum_balance.saturating_sub(operator_history_info.lamports());

    // Transfer
    if required_lamports > 0 {
        invoke(
            &solana_system_interface::instruction::transfer(
                payer.key,
                operator_history_info.key,
                required_lamports,
            ),
            &[payer.clone(), operator_history_info.clone()],
        )?;
    }

    // Allocate space
    let space: u64 = MAX_REALLOC_BYTES;
    invoke_signed(
        &solana_system_interface::instruction::allocate(operator_history_info.key, space),
        &[operator_history_info.clone(), system_program.clone()],
        &[operator_history_seeds
            .iter()
            .map(|seed| seed.as_slice())
            .collect::<Vec<&[u8]>>()
            .as_slice()],
    )?;

    // Assign to the specified program
    invoke_signed(
        &solana_system_interface::instruction::assign(operator_history_info.key, program_id),
        &[operator_history_info.clone(), system_program.clone()],
        &[operator_history_seeds
            .iter()
            .map(|seed| seed.as_slice())
            .collect::<Vec<&[u8]>>()
            .as_slice()],
    )?;

    Ok(())
}
