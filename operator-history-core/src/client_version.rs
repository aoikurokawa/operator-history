use bytemuck::{Pod, Zeroable};
use shank::ShankType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable, ShankType)]
#[repr(C)]
pub struct ClientVersion {
    /// Major
    major: u8,

    /// Minor
    minor: u8,

    /// Patch
    patch: u8,
}

impl Default for ClientVersion {
    fn default() -> Self {
        Self {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl ClientVersion {
    pub const fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Fetch major number
    pub fn major(&self) -> u8 {
        self.major
    }

    /// Fetch minor number
    pub fn minor(&self) -> u8 {
        self.minor
    }

    /// Fetch patch number
    pub fn patch(&self) -> u8 {
        self.patch
    }
}
