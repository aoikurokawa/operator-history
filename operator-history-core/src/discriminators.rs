use jito_bytemuck::Discriminator;

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorHistroyDiscriminator {
    Config = 1,
}

impl Discriminator for Config {
    const DISCRIMINATOR: u8 = OperatorHistroyDiscriminator::Config as u8;
}
