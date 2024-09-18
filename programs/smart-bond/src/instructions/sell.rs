use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), bond_account.id.to_le_bytes().as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Box<Account<'info, BondAccount>>,
}

impl<'info> Sell<'info> {
    pub fn sell_bond(ctx: Context<Sell>, is_for_sale: bool, sale_price: u64) -> Result<()> {
        require!(
            ctx.accounts.bond_account.owner == ctx.accounts.owner.key(),
            BondErrorCode::NotEntitledForSell
        );

        // setting a new bond price
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.is_for_sale = is_for_sale;
        bond_account_data.sale_price = sale_price;

        msg!(
            ">> Bond is open for sale.
            Bond owner :: {0}
            Sale price :: {1}
            Face price :: {2}
            Face mint :: {3}",
            bond_account_data.owner,
            bond_account_data.sale_price,
            bond_account_data.amount_b,
            bond_account_data.mint_b
        );

        Ok(())
    }
}
