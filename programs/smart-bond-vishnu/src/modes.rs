use anchor_lang::prelude::*;
use std::fmt;

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum Convertible {
    //  WhenGraterThan(u64),
    //  WhenLessThan(u64),
    WhenGraterThan { value: u64 },
    WhenLessThan { value: u64 },
}

impl fmt::Display for Convertible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Convertible::WhenGraterThan { value } => write!(f, "> {:?}", value),
            Convertible::WhenLessThan { value } => write!(f, "< {:?}", value),
        }
    }
}
