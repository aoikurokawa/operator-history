use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::loader::{load_signer, load_system_program};
use jito_restaking_core::operator::Operator;
use operator_history_core::{config::Config, get_new_size, operator_history::OperatorHistory};
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program::program::invoke;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_sysvar::Sysvar;

/// Reallocates the [`OperatorHistory`] account
pub fn process_realloc_operator_history_account(
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
    // let operator_data = operator_info.data.borrow();
    // let operator = Operator::try_from_slice_unchecked(&operator_data)?;

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

    if operator_history_info.data_len() < OperatorHistory::SIZE {
        let new_size = get_new_size(operator_history_info.data_len(), OperatorHistory::SIZE)?;
        msg!(
            "Reallocating operator_history from {} bytes to {} bytes",
            operator_history_info.data_len(),
            new_size
        );

        let rent = &Rent::get()?;
        let minimum_balance = rent.minimum_balance(new_size);

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

        operator_history_info.resize(new_size)?;
    }

    // let should_initialize = operator_history_info.data_len() > OperatorHistory::SIZE
    //     && operator_history_info.try_borrow_data()?[0] != OperatorHistory::DISCRIMINATOR;

    // if should_initialize {
    //     let mut operator_history_data = operator_history_info.try_borrow_mut_data()?;
    //     operator_history_data[0] = OperatorHistory::DISCRIMINATOR;
    //     let operator_history =
    //         OperatorHistory::try_from_slice_unchecked_mut(&mut operator_history_data)?;
    //     *operator_history =
    //         OperatorHistory::new(*operator_info.key, operator.index(), operator_history_bump);
    // }

    Ok(())
}
