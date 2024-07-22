use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Repay<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    #[account(
        mut,
        close = issuer, constraint = bond_account.issuer == issuer.key(),
        seeds = ["bond_account".as_bytes(), bond_account.issuer.as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(mut/*, constraint = vault_ata_a.key() == bond_account.vault_key*/)]
    pub vault_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = issuer_ata_a.mint == bond_account.mint_a,
        constraint = issuer_ata_a.mint == vault_ata_a.mint,
        constraint = issuer_ata_a.owner == bond_account.issuer @ BondErrorCode::WrongCollateralRecepient,
        constraint = issuer_ata_a.owner == issuer.key()
    )]
    pub issuer_ata_a: Account<'info, TokenAccount>,
    #[account(mut, 
        constraint = issuer_ata_b.mint == bond_account.mint_b,
        constraint = issuer_ata_b.owner == issuer.key()
    )]
    pub issuer_ata_b: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = owner_ata_b.mint == bond_account.mint_b,
        constraint = owner_ata_b.owner == bond_account.owner @ BondErrorCode::WrongRepaymentRecepient
    )]
    pub owner_ata_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Repay<'info> {
    pub fn repay_bond(ctx: Context<Repay>) -> Result<()> {
        // Transfer issuer's (mint_b) back to the bond owner
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.issuer_ata_b.to_account_info(),
                    to: ctx.accounts.owner_ata_b.to_account_info(),
                    authority: ctx.accounts.issuer.to_account_info(),
                },
            ),
            ctx.accounts.bond_account.amount_b,
        )?;

        // Transfer vault collateral (mint_a) back to issuer
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
                    ctx.accounts.bond_account.issuer.as_ref(),
                    &[ctx.accounts.bond_account.bump],
                ]],
            ),
            ctx.accounts.vault_ata_a.amount,
        )?;

        msg!(
            ">> Bond was successfully repaid.
            Repayment recepient :: {0}
            Repayment ammount :: {1}
            Repayment mint :: {2}",
            ctx.accounts.owner_ata_b.key(),
            ctx.accounts.bond_account.amount_b,
            ctx.accounts.bond_account.mint_b
        );

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
                ctx.accounts.bond_account.issuer.key().as_ref(),
                &[ctx.accounts.bond_account.bump],
            ]],
        ))?;

        msg!(
            ">> Bond was successfully closed.
            Issued by :: {0}
            Closed by  :: {1}",
            ctx.accounts.bond_account.issuer.key(),
            ctx.accounts.issuer.key()
        );
        Ok(())
    }
}
