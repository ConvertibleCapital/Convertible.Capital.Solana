use crate::states::BondAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    // we will use `close` for closing user's bond account.
    #[account(
        mut,
        close = issuer, constraint = bond_account.issuer == issuer.key(),
        seeds = ["bond_account".as_bytes(), bond_account.issuer.as_ref()],
        bump = bond_account.bump,
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(mut, constraint = vault_account.key() == bond_account.issuer)]
    pub vault_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = issuer_ata_a.mint == vault_account.mint,
        constraint = issuer_ata_a.owner == issuer.key()
    )]
    issuer_ata_a: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
}

impl<'info> Cancel<'info> {
    pub fn cancel_bond(ctx: Context<Cancel>) -> Result<()> {
        // return issuer's x_token back to him/her
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.vault_account.to_account_info(),
                    to: ctx.accounts.issuer_ata_a.to_account_info(),
                    authority: ctx.accounts.bond_account.to_account_info(),
                },
                &[&[
                    "bond_account".as_bytes(),
                    ctx.accounts.issuer.key().as_ref(),
                    &[ctx.accounts.bond_account.bump],
                ]],
            ),
            ctx.accounts.vault_account.amount,
        )?;

        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.vault_account.to_account_info(),
                destination: ctx.accounts.issuer.to_account_info(),
                authority: ctx.accounts.bond_account.to_account_info(),
            },
            &[&[
                "bond_account".as_bytes(),
                ctx.accounts.issuer.key().as_ref(),
                &[ctx.accounts.bond_account.bump],
            ]],
        ))?;

        msg!("Bond account transactions cancelled successfully");
        Ok(())
    }
}
