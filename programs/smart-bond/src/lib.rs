use anchor_lang::prelude::*;

declare_id!("AxzjfJo9QFjJm8nkeLrvCYgGg5FxPgNj666foUATE5UU");

#[program]
pub mod smart_bond {
    use super::*;
    pub fn create_bond(ctx: Context<Initialize>, 
        name: String, 
        issuer: String, 
        owner: String,
        currency: String,
        amount: f32,
        collateral_currency: String,
        collateral_amount: f32,
        maturity: String  
    ) -> Result<()> {

        // setting userdata in bonds's account
        let bond_account_data = &mut ctx.accounts.bond_account;
        bond_account_data.bump = *ctx.bumps.get("bond_account").unwrap();

        bond_account_data.authority = *ctx.accounts.signer.key;
        bond_account_data.name = name.to_owned();
        bond_account_data.issuer = issuer.to_owned();
        bond_account_data.owner = owner.to_owned();
        bond_account_data.currency = currency.to_owned();
        bond_account_data.amount = amount.to_owned();
        bond_account_data.collateral_currency  = collateral_currency.to_owned();
        bond_account_data.collateral_amount  = collateral_amount.to_owned();
        bond_account_data.maturity = maturity.to_owned();
        // bond_account_data.conversion_condition = twitter.to_owned();
        // bond_account_data.available = available.to_owned();
        // bond_account_data.price = twitter.to_owned();
        // bond_account_data.is_active = twitter.to_owned();


        // Printing User Info into program's on-chain transaction log.
        msg!("Created a new bond with following details 
            Bond name :: {0}
            Bond owner :: {1}
            Currency :: {2}
            Amount :: {3}
            ", name, owner, currency, amount);
        Ok(())
    }

    pub fn update_bond_owner(ctx: Context<Update>, new_owner: String) -> Result<()> {
        msg!("Updating owner from :: {0} -> to :: {1}", &ctx.accounts.bond_account.owner, &new_owner);
        ctx.accounts.bond_account.owner = new_owner;
        Ok(())
    }

    pub fn delete_bond(_ctx: Context<Close>) -> Result<()> {
        msg!("Bond account closed successfully");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // User's account
    #[account(mut)]
    pub signer: Signer<'info>,
    // Creating a new account for every user with seed of their wallet address.
    // This constraint allow one-account per wallet address
    #[account(
        init, 
        payer = signer, 
        space = BondAccount::LEN, 
        seeds = ["bond_account".as_bytes(), signer.key().as_ref()], 
        bump,
    )] 
    pub bond_account: Account<'info, BondAccount>,
    pub system_program: Program<'info, System>,
}

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

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // we will use `close` for closing user's bond account.
    #[account(
        mut,
        seeds = ["bond_account".as_bytes(), signer.key().as_ref()], 
        bump = bond_account.bump,
        close=signer
    )]
    pub bond_account: Account<'info, BondAccount>,
}


#[account]
pub struct BondAccount {
    pub authority: Pubkey, // Authority of this account
    pub bump: u8,

    pub name: String,                   // Max 10 Chars
    pub issuer: String,                 // Max 10 Chars
    pub owner: String,                  // Max 10 Chars
    pub currency: String,               // Max 10 Chars
    pub amount: f32,                    // 
    pub collateral_currency: String,    // Max 10 Chars
    pub collateral_amount: f32,         // 
    pub maturity: String                // Max 10 Chars
}


impl BondAccount {
    const LEN: usize = 
        8 +         // discriminator
        32 +        // Pubkey
        1 +         // bump

        (4 + 10) +  // 10 chars of Name
        (4 + 10) +  // 10 chars of issuer  
        (4 + 10) +  // 10 chars of owner  
        (4 + 10) +  // 10 chars of currency  
        (4) +       // 4 of amount
        (4 + 10) +  // 10 chars of collateral_currency 
        (4) +       // 4 of collateral_amount
        (4 + 10);   // 10 chars of maturity 
}

