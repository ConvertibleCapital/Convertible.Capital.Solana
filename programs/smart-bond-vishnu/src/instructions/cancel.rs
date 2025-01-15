use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    #[account(
        mut,
        close = issuer, constraint = bond_account.issuer == issuer.key(),
        seeds = ["bond_account".as_bytes(), bond_account.id.to_le_bytes().as_ref()],
        bump = bond_account.bump,
    )]
    pub bond_account: Box<Account<'info, BondAccount>>,
    #[account(mut/*, constraint = vault_ata_a.key() == bond_account.vault_key*/)]
    pub vault_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = issuer_ata_a.mint == vault_ata_a.mint,
        constraint = issuer_ata_a.owner == issuer.key()
    )]
    pub issuer_ata_a: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Cancel<'info> {
    pub fn cancel_bond(ctx: Context<Cancel>) -> Result<()> {
        require!(
            ctx.accounts.bond_account.issuer.key() == ctx.accounts.bond_account.owner.key(),
            BondErrorCode::NotForClosure
        );

        // Return issuer's collateral (mint_a) back
        anchor_spl::token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.vault_ata_a.to_account_info(),
                    to: ctx.accounts.issuer_ata_a.to_account_info(),
                    authority: ctx.accounts.bond_account.to_account_info(),
                },
                &[&[
                    "bond_account".as_bytes(),
                    ctx.accounts.bond_account.id.to_le_bytes().as_ref(),
                    &[ctx.accounts.bond_account.bump],
                ]],
            ),
            ctx.accounts.vault_ata_a.amount,
        )?;

        // Close vault account
        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.vault_ata_a.to_account_info(),
                destination: ctx.accounts.issuer.to_account_info(),
                //destination: ctx.accounts.issuer_ata_a.to_account_info(),
                authority: ctx.accounts.bond_account.to_account_info(),
            },
            &[&[
                "bond_account".as_bytes(),
                ctx.accounts.bond_account.id.to_le_bytes().as_ref(),
                &[ctx.accounts.bond_account.bump],
            ]],
        ))?;

        msg!(
            ">> Bond was successfully closed.
            Issued by :: {0}
            Closed by :: {1}",
            ctx.accounts.bond_account.issuer.key(),
            ctx.accounts.issuer.key()
        );
        Ok(())
    }
}
