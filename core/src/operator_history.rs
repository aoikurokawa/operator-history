use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{
    types::{PodU32, PodU64},
    AccountDeserialize, Discriminator,
};
use shank::ShankAccount;
use solana_account_info::AccountInfo;
use solana_msg::msg;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::circ_buf::CircBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct OperatorHistory {
    /// Jito Restaking [`Operator`] account pubkey
    operator_account: Pubkey,

    /// Index of operator of all OperatorHistory accounts
    index: PodU64,

    /// Struct version
    struct_version: PodU32,

    /// Bump
    bump: u8,

    /// History
    history: CircBuf,

    /// Reserved space
    reserved_space: [u8; 328],
}

impl OperatorHistory {
    pub const SIZE: usize = 8 + size_of::<Self>();

    pub fn new(operator_account: Pubkey, index: u64, bump: u8) -> Self {
        Self {
            operator_account,
            struct_version: PodU32::from(0),
            index: PodU64::from(index),
            bump,
            history: CircBuf::default(),
            reserved_space: [0; 328],
        }
    }

    pub fn initialize(&mut self, operator: &Pubkey, index: u64, bump: u8) {
        self.operator_account = *operator;
        self.index = PodU64::from(index);
        self.struct_version = PodU32::from(0);
        self.bump = bump;
        self.history = CircBuf::default();
        self.reserved_space = [0; 328];
    }

    /// Operator account
    pub const fn operator_account(&self) -> Pubkey {
        self.operator_account
    }

    /// Index
    pub fn index(&self) -> u64 {
        self.index.into()
    }

    /// Struct version
    pub fn struct_version(&self) -> u32 {
        self.struct_version.into()
    }

    /// History
    pub const fn history(&self) -> CircBuf {
        self.history
    }

    /// Returns the seeds for the PDA
    pub fn seeds(operator: &Pubkey) -> Vec<Vec<u8>> {
        vec![b"operator_history".to_vec(), operator.to_bytes().to_vec()]
    }

    /// Find the program address for the [`OperatorHistory`] account.
    pub fn find_program_address(
        program_id: &Pubkey,
        operator: &Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(operator);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Attempts to load the account as [`OperatorHistory`], returning an error if it's not valid.
    pub fn load(
        program_id: &Pubkey,
        operator: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("OperatorHistory account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("OperatorHistory account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("OperatorHistory account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("OperatorHistory account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, operator).0)
        {
            msg!("OperatorHistory account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(())
    }
}
