use crate::states::BondAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // we will use `close` for closing user's bond account.
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), signer.key().as_ref()], 
        bump = bond_account.bump,
        close=signer
    )]
    pub bond_account: Account<'info, BondAccount>,
}

impl<'info> Close<'info> {
    pub fn delete_bond(_ctx: Context<Close>) -> Result<()> {
        msg!("Bond account closed successfully");
        Ok(())
    }
}
