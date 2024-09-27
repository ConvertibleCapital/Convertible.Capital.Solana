#![allow(clippy::result_large_err)]
use anchor_lang::prelude::*;
use instructions::*;
use modes::*;

pub mod errors;
pub mod instructions;
pub mod modes;
pub mod states;

declare_id!("97USSTPNysfsokmHgYjfMzZxHYy4GHhK419vJ8Kk6sdt");

#[program]
pub mod smart_bond {

    use super::*;

    pub fn create_bond(
        ctx: Context<Create>,
        bond_id: u64,
        //bumps: &InitializeBumps,
        name: String,
        amount_a: u64, //collateral
        amount_b: u64, //loan (face)
        maturity_date: i64,
        is_for_sale: bool,
        sale_message: String,
        price_feed: String,
        convertible: Convertible,
    ) -> Result<()> {
        Create::create_bond(
            ctx,
            bond_id,
            /*bumps,*/
            name,
            amount_a,
            amount_b,
            maturity_date,
            is_for_sale,
            sale_message,
            price_feed,
            convertible,
        )
    }

    pub fn cancel_bond(ctx: Context<Cancel>) -> Result<()> {
        Cancel::cancel_bond(ctx)
    }

    pub fn sell_bond(
        ctx: Context<Sell>,
        is_for_sale: bool,
        sale_price: u64,
        sale_message: String,
    ) -> Result<()> {
        Sell::sell_bond(ctx, is_for_sale, sale_price, sale_message)
    }

    pub fn buy_bond(ctx: Context<Buy>) -> Result<()> {
        Buy::buy_bond(ctx)
    }

    pub fn convert_bond(ctx: Context<Convert>) -> Result<()> {
        Convert::convert_bond(ctx)
    }

    pub fn check_bond(ctx: Context<Pyth>) -> Result<()> {
        Pyth::check_bond(ctx)
    }

    pub fn repay_bond(ctx: Context<Repay>) -> Result<()> {
        Repay::repay_bond(ctx)
    }
}
