use crate::states::BondAccount;
use crate::{errors::BondErrorCode, modes::Convertible};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct Create<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        mut,
        constraint = issuer_ata_a.mint == mint_a.key(),
        constraint = issuer_ata_a.owner == issuer.key()
    )]
    pub issuer_ata_a: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = issuer,
        space = 8 + BondAccount::INIT_SPACE,
        seeds = ["bond_account".as_bytes(), &id.to_le_bytes()],
        //seeds = ["bond_account".as_bytes(), issuer.key().as_ref()],
        bump
    )]
    pub bond_account: Box<Account<'info, BondAccount>>,
    #[account(
        init_if_needed,
        payer = issuer,
        associated_token::mint = mint_a,
        associated_token::authority = bond_account,
    )]
    pub vault_ata_a: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Create<'info> {
    pub fn create_bond(
        ctx: Context<Create>,
        id: u64,
        name: String,
        amount_a: u64,
        amount_b: u64,
        maturity_date: i64,
        is_for_sale: bool,
        price_feed: Pubkey,
        convertible: Convertible,
    ) -> Result<()> {
        require!(
            ctx.accounts.vault_ata_a.amount < amount_a,
            BondErrorCode::BondAlreadyExists
        );

        // Saving info into program's on-chain bond_account.
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.id = id;
        bond_account_data.bump = ctx.bumps.bond_account;
        bond_account_data.issuer = ctx.accounts.issuer.key();
        bond_account_data.owner = ctx.accounts.issuer.key();
        bond_account_data.vault = ctx.accounts.vault_ata_a.key();

        bond_account_data.name = name;
        bond_account_data.amount_a = amount_a;
        bond_account_data.mint_a = ctx.accounts.mint_a.key();
        bond_account_data.amount_b = amount_b;
        bond_account_data.mint_b = ctx.accounts.mint_b.key();
        bond_account_data.maturity_date = maturity_date;
        bond_account_data.is_for_sale = is_for_sale;
        bond_account_data.sale_price = amount_b;
        bond_account_data.price_feed = price_feed;
        bond_account_data.is_convertible = false;
        bond_account_data.convertible = convertible;

        msg!(
            ">> Bond was successfully created. 
            Bond name :: {0}
            Bond owner :: {1}
            Face amount :: {2}
            Mint :: {3}",
            bond_account_data.name,
            bond_account_data.owner,
            bond_account_data.amount_b,
            bond_account_data.mint_b
        );

        // Transfer issuer's mint_a in program owned escrow token account
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.issuer_ata_a.to_account_info(),
                    to: ctx.accounts.vault_ata_a.to_account_info(),
                    authority: ctx.accounts.issuer.to_account_info(),
                },
            ),
            amount_a,
        )?;

        msg!(
            ">> Bonds was collateralized. 
            Vault account :: {0}
            Vault owner :: {1}
            Vault mint :: {2}",
            ctx.accounts.vault_ata_a.key(),
            ctx.accounts.vault_ata_a.owner,
            ctx.accounts.vault_ata_a.mint
        );

        Ok(())
    }
}
