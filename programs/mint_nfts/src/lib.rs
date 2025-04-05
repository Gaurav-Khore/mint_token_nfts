use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token_interface::{Mint, TokenAccount, TokenInterface}};

declare_id!("2KVpzmgNNyhL2qnfists6uxjsAytJdxNdUaBEML77rBU");

#[program]
pub mod mint_nfts {

    use anchor_spl::metadata::{create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3};

    use super::*;

    //mint created successfully
    pub fn create_mint(ctx: Context<CreateMint>,token_name: String, token_symbol: String, token_uri: String) -> Result<()> {
        msg!("token with name... {:?}",token_name);
        msg!("Token symbol... {:?}",token_symbol);

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3{
                    metadata: ctx.accounts.metadata_account.to_account_info(), //metadata account
                    mint: ctx.accounts.mint.to_account_info(), //mint account
                    mint_authority: ctx.accounts.signer.to_account_info(), // mint authority
                    payer: ctx.accounts.signer.to_account_info(), // payer
                    update_authority: ctx.accounts.signer.to_account_info(), // update authority
                    system_program: ctx.accounts.system_program.to_account_info(), //system program
                    rent: ctx.accounts.rent.to_account_info(), // rent account
                }
            ), 
            DataV2 { 
                name: token_name, 
                symbol: token_symbol, 
                uri: token_uri, 
                seller_fee_basis_points: 0, 
                creators: None, 
                collection: None, 
                uses: None 
            }, 
            false, 
            true,
            None
        )?;

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

    //metadata account
    /// CHECK: Validate address by deriving pda
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    //mint
    #[account(
        init_if_needed,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key(),
        seeds = [b"mint1"],
        bump
    )]
    pub mint: InterfaceAccount<'info,Mint>,
    pub token_program: Interface<'info,TokenInterface>,
    pub token_metadata_program: Program<'info,Metadata>,
    pub system_program: Program<'info,System>,

    pub rent: Sysvar<'info,Rent>
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