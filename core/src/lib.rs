pub mod circ_buf;
pub mod client_version;
pub mod config;
pub mod discriminators;
pub mod operator_history;
pub mod operator_history_entry;

pub(crate) const OPERATOR_HISTORY_ENTRY_MAX_ITEMS: usize = 512;
