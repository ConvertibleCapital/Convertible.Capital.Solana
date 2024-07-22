use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Convert<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        close = owner, constraint = bond_account.owner == owner.key(),
        seeds = ["bond_account".as_bytes(), bond_account.issuer.as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(mut/*, constraint = vault_ata_a.key() == bond_account.vault_key*/)]
    pub vault_ata_a: Account<'info, TokenAccount>,
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
                    from: ctx.accounts.vault_ata_a.to_account_info(),
                    to: ctx.accounts.owner_ata_a.to_account_info(),
                    authority: ctx.accounts.bond_account.to_account_info(),
                },
                &[&[
                    "bond_account".as_bytes(),
                    ctx.accounts.bond_account.issuer.as_ref(),
                    &[ctx.accounts.bond_account.bump],
                ]],
            ),
            ctx.accounts.vault_ata_a.amount,
        )?;

        msg!(
            ">> Bond was successfully converted.
            Collateral owner :: {0}
            Collateral ammount :: {1}
            Collateral mint :: {2}",
            ctx.accounts.owner_ata_a.key(),
            ctx.accounts.bond_account.amount_a,
            ctx.accounts.bond_account.mint_a
        );

        // Close vault account
        anchor_spl::token::close_account(CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::CloseAccount {
                account: ctx.accounts.vault_ata_a.to_account_info(),
                destination: ctx.accounts.owner.to_account_info(),
                //destination: ctx.accounts.issuer_ata_a.to_account_info(),
                authority: ctx.accounts.bond_account.to_account_info(),
            },
            &[&[
                "bond_account".as_bytes(),
                ctx.accounts.bond_account.issuer.key().as_ref(),
                &[ctx.accounts.bond_account.bump],
            ]],
        ))?;

        msg!(
            ">> Bond was successfully closed.
            Issued by :: {0}
            Closed by  :: {1}",
            ctx.accounts.bond_account.issuer.key(),
            ctx.accounts.owner.key()
        );
        Ok(())
    }
}
