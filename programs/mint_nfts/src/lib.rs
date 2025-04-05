use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

declare_id!("2KVpzmgNNyhL2qnfists6uxjsAytJdxNdUaBEML77rBU");

#[program]
pub mod mint_nfts {
    use super::*;

    //mint created successfully
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        msg!("Created Mint Successfully... {:?}",ctx.accounts.mint.key());
        Ok(())
    }

    //create token account
    pub fn create_token_account(ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!("Token Account created successfully ... {:?}",ctx.accounts.token_account.key());
        Ok(())
    }

    //create token account using associated
    pub fn create_token_account_associated(ctx: Context<CreateTokenAccountAssociated>) -> Result<()> {
        msg!("Token Account Created Successfully associated .. {:?}",ctx.accounts.token_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        mint::decimals = 6,
        mint::authority = mint.key(),
        mint::freeze_authority = mint.key(),
        seeds = [b"mint"],
        bump
    )]
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program: Program<'info,System>
}


// token account using pda
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        token::mint = mint,
        token::authority = token_account,
        token::token_program = token_program,
        seeds = [b"token"],
        bump
    )]
    pub token_account : InterfaceAccount<'info,TokenAccount>,
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub system_program : Program<'info,System>
}


// create token account using associated token account
#[derive(Accounts)]
pub struct CreateTokenAccountAssociated<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub token_account : InterfaceAccount<'info,TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program : Interface<'info, TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>
}