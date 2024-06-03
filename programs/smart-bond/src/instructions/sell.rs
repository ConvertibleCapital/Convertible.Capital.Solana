use crate::states::BondAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), signer.key().as_ref()], 
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
}

impl<'info> Sell<'info> {
    // require!(
    //     ctx.accounts.bond_account.owner == signer.key(),
    //     BondErrorCode::NotEntitledForSell
    // );

    pub fn sell_bond(ctx: Context<Sell>, is_for_sale: bool, sale_price: u64) -> Result<()> {
        // setting a new bond price
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.is_for_sale = is_for_sale;
        bond_account_data.sale_price = sale_price;

        msg!("Bond was successfully opened for sale.");
        msg!(
            "Bond was successfully opened for sale.
            Sale price :: {0}",
            sale_price
        );

        Ok(())
    }
}
