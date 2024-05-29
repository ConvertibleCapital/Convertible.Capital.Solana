use crate::states::BondAccount;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // User's bond account
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), signer.key().as_ref()], 
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
}

impl<'info> Update<'info> {
    pub fn update_bond_owner(ctx: Context<Update>, new_owner: Pubkey) -> Result<()> {
        msg!(
            "Updating owner from :: {0} -> to :: {1}",
            &ctx.accounts.bond_account.owner,
            &new_owner
        );
        ctx.accounts.bond_account.owner = new_owner;
        Ok(())
    }
}
