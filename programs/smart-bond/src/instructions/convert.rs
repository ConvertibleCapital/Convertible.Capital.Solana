use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Convert<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), bond_account.issuer.as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(mut/*, constraint = vault_account.key() == bond_account.vault_key*/)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(mut, 
        constraint = owner_ata_a.mint == bond_account.mint_a,
        constraint = owner_ata_a.owner == bond_account.owner @ BondErrorCode::NotEntitledForConversion,
        /*constraint = owner_ata_a.owner == owner.key()*/
    )]
    pub owner_ata_a: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Convert<'info> {
    pub fn convert_bond(ctx: Context<Convert>) -> Result<()> {
        require!(
            ctx.accounts.bond_account.is_convertible == true,
            BondErrorCode::NonConvertible
        );

        // Transfer collateral (mint_a) to (owner)
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.vault_account.to_account_info(),
                    to: ctx.accounts.owner_ata_a.to_account_info(),
                    authority: ctx.accounts.bond_account.to_account_info(),
                },
                &[&[
                    "bond_account".as_bytes(),
                    ctx.accounts.bond_account.issuer.as_ref(),
                    &[ctx.accounts.bond_account.bump],
                ]],
            ),
            ctx.accounts.vault_account.amount,
        )?;

        msg!(
            ">> Bond was successfully converted.
            Collateral owner :: {0}
            Collateral ammount :: {1}
            Collateral mint :: {2}",
            ctx.accounts.bond_account.owner,
            ctx.accounts.vault_account.amount,
            ctx.accounts.vault_account.mint
        );

        Ok(())
    }
}
