use bytemuck::{Pod, Zeroable};
use jito_bytemuck::AccountDeserialize;
use jito_bytemuck::Discriminator;
use shank::ShankAccount;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

const RESERVED_SPACE_LEN: usize = 288;

/// The global configuration account for the operator ledger program.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct Config {
    /// The jito restaking program id
    jito_restaking_program_id: Pubkey,

    /// The pubkey of admin
    admin: Pubkey,

    /// The bump seed for the PDA
    bump: u8,

    /// Reserved space
    reserved: [u8; 288],
}

impl Config {
    /// Construct new config
    pub const fn new(jito_restaking_program_id: Pubkey, admin: Pubkey, bump: u8) -> Self {
        Self {
            jito_restaking_program_id,
            admin,
            bump,
            reserved: [0; RESERVED_SPACE_LEN],
        }
    }

    /// Jito Restaking Program ID
    pub const fn jito_restaking_program_id(&self) -> Pubkey {
        self.jito_restaking_program_id
    }

    /// Admin pubkey
    pub const fn admin(&self) -> Pubkey {
        self.admin
    }

    /// Returns the seeds for the PDA
    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"config".to_vec()]
    }

    /// Find the program address for the global configuration account
    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds();
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Attempts to load the account as [`Config`], returning an error if it's not valid.
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.key.ne(&Self::find_program_address(program_id).0) {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
