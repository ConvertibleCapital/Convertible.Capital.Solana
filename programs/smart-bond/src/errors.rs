use anchor_lang::prelude::*;

#[error_code]
pub enum PriceErrorCode {
    #[msg("Could not load price account")]
    PythError,
    #[msg("Failed to serialize price account")]
    TryToSerializePriceAccount,
    #[msg("Invalid argument provided")]
    InvalidArgument,
}
