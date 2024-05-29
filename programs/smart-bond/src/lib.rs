#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod states;
pub mod errors;

declare_id!("97USSTPNysfsokmHgYjfMzZxHYy4GHhK419vJ8Kk6sdt");

#[program]
pub mod smart_bond {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        // bumps: &InitializeBumps,
        name: String,
        amount_a: u64, //collateral
        amount_b: u64, //loan (face)
        maturity: String,
    ) -> Result<()> {
        Initialize::create_bond(
            ctx, seed, /*bumps,*/ name, amount_a, amount_b, maturity,
        )
        //ctx.accounts.(seed, &ctx.bumps, initializer_amount, taker_amount)?;
        //ctx.accounts.deposit(initializer_amount)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        //ctx.accounts.delete_bond(ctx)

        Close::delete_bond(ctx) 
    }

    // pub fn update(ctx: Context<Exchange>) -> Result<()> {
    //     ctx.accounts.update_bond_owner()
    // }

    pub fn buy(ctx: Context<Buy>) -> Result<()> {
        //ctx.accounts.deposit()?;
        //ctx.accounts.withdraw_and_close_vault()
        Buy::buy_bond(ctx)
    }

    pub fn convert(ctx: Context<Convert>) -> Result<()> {
        Convert::convert_bond(ctx)
    }

    pub fn price(ctx: Context<Pyth>) -> Result<()> {
        //PythRequest::read_price(ctx)
        Pyth::read_price(ctx)
    }
}
