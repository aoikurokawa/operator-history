use jito_bytemuck::Discriminator;

use crate::{config::Config, operator_history::OperatorHistory};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorHistroyDiscriminator {
    /// Config
    Config = 1,

    /// Operator History
    OperatorHistory,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = OperatorHistroyDiscriminator::Config as u8;
}

impl Discriminator for OperatorHistory {
    const DISCRIMINATOR: u8 = OperatorHistroyDiscriminator::OperatorHistory as u8;
}
