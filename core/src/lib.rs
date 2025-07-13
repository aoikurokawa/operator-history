use solana_account_info::MAX_PERMITTED_DATA_INCREASE;
use solana_program_error::ProgramError;

pub mod circ_buf;
pub mod client_version;
pub mod config;
pub mod discriminators;
pub mod operator_history;
pub mod operator_history_entry;

pub(crate) const OPERATOR_HISTORY_ENTRY_MAX_ITEMS: usize = 512;
pub const MAX_REALLOC_BYTES: u64 = MAX_PERMITTED_DATA_INCREASE as u64;

/// Calculate new size for reallocation, capped at target size
/// Returns the minimum of (current_size + MAX_REALLOC_BYTES) and target_size
pub fn get_new_size(current_size: usize, target_size: usize) -> Result<usize, ProgramError> {
    Ok(current_size
        .checked_add(MAX_REALLOC_BYTES as usize)
        .ok_or(ProgramError::ArithmeticOverflow)?
        .min(target_size))
}
