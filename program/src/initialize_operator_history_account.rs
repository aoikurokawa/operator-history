use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use jito_restaking_core::operator::Operator;
use operator_history_core::{config::Config, operator_history::OperatorHistory};
use operator_history_sdk::error::OperatorHistoryError;
use solana_account_info::AccountInfo;
use solana_msg::msg;
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
    let operator_data = operator_info.data.borrow();
    let operator = Operator::try_from_slice_unchecked(&operator_data)?;

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
    create_account(
        payer,
        operator_history_info,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(size_of::<OperatorHistory>() as u64)
            .ok_or(OperatorHistoryError::Arithmetic)?,
        &operator_history_seeds,
    )?;

    let mut operator_history_data = operator_history_info.try_borrow_mut_data()?;
    operator_history_data[0] = OperatorHistory::DISCRIMINATOR;
    let operator_history =
        OperatorHistory::try_from_slice_unchecked_mut(&mut operator_history_data)?;
    *operator_history =
        OperatorHistory::new(*operator_info.key, operator.index(), operator_history_bump);

    Ok(())
}
