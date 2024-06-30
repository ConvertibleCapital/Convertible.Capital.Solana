use crate::modes::Convertible;
use crate::states::BondAccount;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(seed: u64, issuer_amount: u64)]
pub struct Create<'info> {
    #[account(mut)]
    pub issuer: Signer<'info>,
    pub mint_a: Account<'info, Mint>,
    pub mint_b: Account<'info, Mint>,
    #[account(
        mut,
        constraint = issuer_ata_a.mint == mint_a.key(),
        constraint = issuer_ata_a.mint == vault_account.mint,
        constraint = issuer_ata_a.owner == issuer.key()
    )]
    pub issuer_ata_a: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = issuer,
        space = BondAccount::INIT_SPACE,
        //seeds = [b"bond_account".as_ref(), &seed.to_le_bytes()],
        seeds = ["bond_account".as_bytes(), issuer.key().as_ref()],
        bump
    )]
    pub bond_account: Account<'info, BondAccount>,
    #[account(
        init,
        payer = issuer,
        token::mint = mint_a,
        token::authority = bond_account,
    )]
    pub vault_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    pub fn create_bond(
        ctx: Context<Create>,
        seed: u64,
        // bumps: &InitializeBumps,
        name: String,
        amount_a: u64,
        amount_b: u64,
        maturity_date: i64,
        is_for_sale: bool,
        price_feed: Pubkey,
        convertible: Convertible,
    ) -> Result<()> {
        // Saving info into program's on-chain bond_account.
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.seed = seed;
        bond_account_data.bump = ctx.bumps.bond_account;
        bond_account_data.issuer = ctx.accounts.issuer.key();
        bond_account_data.owner = ctx.accounts.issuer.key();

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
                    to: ctx.accounts.vault_account.to_account_info(),
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
            ctx.accounts.vault_account.key(),
            ctx.accounts.vault_account.owner,
            ctx.accounts.vault_account.mint
        );

        Ok(())
    }
}
