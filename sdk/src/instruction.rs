use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum OperatorHistoryInstruction {
    /// Initialize the global configuration
    #[account(0, writable, name = "config")]
    #[account(1, writable, signer, name = "admin")]
    #[account(2, name = "jito_restaking_program")]
    #[account(3, name = "system_program")]
    InitializeConfig,

    /// Initialize the operator history account
    #[account(0, name = "config")]
    #[account(1, writable, name = "operator_history")]
    #[account(2, name = "operator")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "system_program")]
    InitializeOperatorHistoryAccount,

    /// Reallocate the operator history account
    #[account(0, name = "config")]
    #[account(1, writable, name = "operator_history")]
    #[account(2, name = "operator")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "system_program")]
    ReallocOperatorHistoryAccount,
}
