use crate::{errors::BondErrorCode, states::BondAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), bond_account.id.to_le_bytes().as_ref()],
        bump = bond_account.bump
    )]
    pub bond_account: Box<Account<'info, BondAccount>>,
    #[account(mut/*, constraint = vault_ata_a.key() == bond_account.vault_key*/)]
    pub vault_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = owner_ata_b.mint == bond_account.mint_b,
        constraint = owner_ata_b.owner == bond_account.owner
    )]
    pub owner_ata_b: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = buyer_ata_b.mint == bond_account.mint_b,
        constraint = buyer_ata_b.owner == buyer.key()
    )]
    pub buyer_ata_b: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Buy<'info> {
    pub fn buy_bond(ctx: Context<Buy>) -> Result<()> {
        require!(
            ctx.accounts.bond_account.is_for_sale == true,
            BondErrorCode::NonForSale
        );

        // Transfer buyers deposit (mint_b) to issurer
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.buyer_ata_b.to_account_info(),
                    to: ctx.accounts.owner_ata_b.to_account_info(),
                    authority: ctx.accounts.buyer.to_account_info(),
                },
            ),
            //ctx.accounts.bond_account.amount_b,
            ctx.accounts.bond_account.sale_price,
        )?;

        // Setting a new bond owner
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.owner = ctx.accounts.buyer.key();
        bond_account_data.is_for_sale = false;

        msg!(
            ">> Bond was successfully sold.
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
