use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BondAccount {
    pub seed: u64,
    pub bump: u8,
    pub issuer: Pubkey,
    pub owner: Pubkey,

    #[max_len(32)]
    pub name: String,
    pub amount_b: u64,
    pub mint_b: Pubkey,
    pub amount_a: u64,
    pub mint_a: Pubkey,
    #[max_len(10)]
    pub maturity: String,
}

// impl Space for BondAccount {
//     const INIT_SPACE: usize = 8 +         // discriminator
//         8 +         // seed
//         1 +         // bump
//         32 +        // issuer
//         32 +        // owner

//         (4 + 32) +  // 32 chars of name (bond name)
//         8 +         // amount
//         32 +        // mint
//         8 +         // collateral_amount
//         32 +        // collateral_mint
//         (4 + 10); // 10 chars of maturity
// }
