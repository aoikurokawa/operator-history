use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{types::PodU32, AccountDeserialize};
use shank::ShankAccount;
use solana_pubkey::Pubkey;

use crate::circ_buf::CircBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, AccountDeserialize, ShankAccount)]
#[repr(C)]
pub struct OperatorHistory {
    /// Jito Restaking [`Operator`] account pubkey
    operator_account: Pubkey,

    /// Struct version
    struct_version: PodU32,

    /// Index of operator of all OperatorHistory accounts
    index: PodU32,

    /// Bump
    bump: u8,

    /// History
    history: CircBuf,

    /// Reserved space
    reserved_space: [u8; 328],
}
