use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_jsm_core::{
    create_account,
    loader::{load_signer, load_system_account, load_system_program},
};
use operator_history_core::config::Config;
use operator_history_sdk::error::OperatorHistoryError;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_error::{ProgramError, ProgramResult};
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_sysvar::Sysvar;

/// Initializes the global configuration for the operator history program
pub fn process_initialize_config(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("Read account");
    let [config, admin, jito_restaking_program, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    msg!("Check account");

    load_system_account(config, true)?;
    load_signer(admin, true)?;
    load_system_program(system_program)?;

    msg!("Check config account");

    // The Config shall be at the canonical PDA
    let (config_pubkey, config_bump, mut config_seeds) = Config::find_program_address(program_id);
    config_seeds.push(vec![config_bump]);
    if config.key.ne(&config_pubkey) {
        msg!("Config account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("Initializing config at address {}", config.key);
    create_account(
        admin,
        config,
        system_program,
        program_id,
        &Rent::get()?,
        8_u64
            .checked_add(size_of::<Config>() as u64)
            .ok_or(OperatorHistoryError::Arithmetic)?,
        &config_seeds,
    )?;

    let mut config_data = config.try_borrow_mut_data()?;
    config_data[0] = Config::DISCRIMINATOR;
    let config = Config::try_from_slice_unchecked_mut(&mut config_data)?;
    *config = Config::new(*jito_restaking_program.key, *admin.key, config_bump);

    Ok(())
}
