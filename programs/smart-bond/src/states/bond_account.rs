use crate::modes::Convertible;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BondAccount {
    pub id: u64,
    pub bump: u8,
    pub issuer: Pubkey,
    pub owner: Pubkey,
    pub vault: Pubkey,

    #[max_len(32)]
    pub name: String,
    pub amount_b: u64,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub mint_a: Pubkey,
    pub maturity_date: i64,
    pub is_for_sale: bool,
    pub sale_price: u64,
    pub price_feed: Pubkey,
    pub is_convertible: bool,
    pub convertible: Convertible,
}

// impl Space for BondAccount {
//     const INIT_SPACE: usize = 8 +         // discriminator
//         8 +         // id
//         1 +         // bump
//         32 +        // issuer
//         32 +        // owner
//         32 +        // vault

//         (4 + 32) +  // 32 chars of name (bond name)
//         8 +         // amount
//         32 +        // mint
//         8 +         // collateral_amount
//         32 +        // collateral_mint
//         8 +         // maturity
//         ? +         // is_for_sale
//                     //sale_price
//                     //price_feed
//                     //is_convertible
//         ? +         // convertible_price
//         ? +         // convertible_mode
// }
