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

#[error_code]
pub enum BondErrorCode {
    #[msg("Bond could not be converted now")]
    NonConvertible,
    #[msg("Bond is not opened for sale")]
    NonForSale,
    #[msg("Only bond owner can open bond for sale")]
    NotEntitledForSell,
    #[msg("Bond can not be closed after sale")]
    NotForClosure,
    #[msg("Only bond owner can convert the bond")]
    NotEntitledForConversion,
    #[msg("Repayment recepient must be a bond owner")]
    WrongRepaymentRecepient,
    #[msg("Collateral recepient must be a bond issuer")]
    WrongCollateralRecepient,
    #[msg("Bond already exists and collateralized")]
    BondAlreadyExists,
}
