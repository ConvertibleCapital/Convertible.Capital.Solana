use crate::states::BondAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // we will use `buy` for buying user's bond account.
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), bond_account.issuer.as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(mut/* , constraint = vault_account.key() == bond_account.owner*/)]
    pub vault_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = issuer_ata_b.mint == bond_account.mint_b)]
    pub issuer_ata_b: Account<'info, TokenAccount>,

    #[account(mut, constraint = owner_ata_a.mint == bond_account.mint_a)]
    pub owner_ata_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = owner_ata_b.mint == bond_account.mint_b,
        constraint = owner_ata_b.owner == owner.key()
    )]
    pub owner_ata_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Buy<'info> {
    pub fn buy_bond(ctx: Context<Buy>) -> Result<()> {
        // transfer owners's deposit (mint_b) to issurer
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.owner_ata_b.to_account_info(),
                    to: ctx.accounts.issuer_ata_b.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            ctx.accounts.bond_account.amount_b,
        )?;

        // setting a new bond owner
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.owner = ctx.accounts.owner.key();
        msg!(
            "Bond was sold to a new owner :: {0}",
            bond_account_data.owner
        );

        Ok(())
    }
}
