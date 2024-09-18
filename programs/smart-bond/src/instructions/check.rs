#![allow(clippy::result_large_err)]

use crate::errors::{BondErrorCode, PriceErrorCode};
use crate::modes::Convertible;
use crate::states::{BondAccount, PriceFeed};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Pyth<'info> {
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), bond_account.id.to_le_bytes().as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Box<Account<'info, BondAccount>>,
    #[account(
        address = bond_account.price_feed @ PriceErrorCode::InvalidArgument
    )]
    pub price_feed: Account<'info, PriceFeed>,
    pub clock: Sysvar<'info, Clock>,
    pub system_program: Program<'info, System>,
}

impl<'info> Pyth<'info> {
    pub fn check_bond(ctx: Context<Pyth>) -> Result<()> {
        let clock = &ctx.accounts.clock;
        let timestamp: i64 = clock.unix_timestamp;

        let price_feed = &ctx.accounts.price_feed;
        let bond_account_data = &mut ctx.accounts.bond_account;

        // When maturity date has passed - set bond as convertible.
        if timestamp > bond_account_data.maturity_date {
            bond_account_data.is_convertible = true;
            return Ok(());
        }

        // Load the price from the price feed.
        let price: pyth_sdk::Price = price_feed
            .get_price_no_older_than(timestamp, 60)
            .ok_or(PriceErrorCode::PythError)?;

        let confidence_interval: u64 = price.conf;
        let asset_price_full: i64 = price.price;
        let asset_exponent: i32 = price.expo;
        let asset_price = asset_price_full as f64 * 10f64.powi(asset_exponent);

        msg!(
            ">> Market data check.
            Timestamp :: {0}
            Convertible at :: {1}
            Current price :: {2}
            Confidence interval :: {3}",
            timestamp,
            bond_account_data.convertible,
            asset_price,
            confidence_interval
        );

        // When market price matches the condition - set bond as convertible.
        let instruction = bond_account_data.convertible;
        match instruction {
            Convertible::WhenGraterThan { value } => {
                if asset_price > (value as f64) {
                    bond_account_data.is_convertible = true;
                    return Ok(());
                }
            }
            Convertible::WhenLessThan { value } => {
                if asset_price > (value as f64) {
                    bond_account_data.is_convertible = true;
                    return Ok(());
                }
            }
        }

        return err!(BondErrorCode::NonConvertible);
    }
}

// pub mod pythexample {
//     use super::*;
//     pub fn read_price(ctx: Context<Pyth>) -> Result<()> {
//         let price_feed = &ctx.accounts.price_feed;
//         let clock = &ctx.accounts.clock;
//         // Get the current timestamp
//         let timestamp: i64 = clock.unix_timestamp;
//         // Load the price from the price feed. Here, the price can be no older than 500 seconds.
//         let price: pyth_sdk::Price = price_feed
//             .get_price_no_older_than(timestamp, 30)
//             .ok_or(PriceErrorCode::PythError)?;

//         let confidence_interval: u64 = price.conf;

//         let asset_price_full: i64 = price.price;

//         let asset_exponent: i32 = price.expo;

//         let asset_price = asset_price_full as f64 * 10f64.powi(asset_exponent);

//         msg!("Price: {}", asset_price);
//         msg!("Confidence interval: {}", confidence_interval);

//         Ok(())
//     }
// }
