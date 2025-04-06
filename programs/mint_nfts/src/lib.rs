use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::Metadata, token::{self, Token}, token_interface::{Mint, MintTo, TokenAccount, TokenInterface}};

declare_id!("2KVpzmgNNyhL2qnfists6uxjsAytJdxNdUaBEML77rBU");

#[program]
pub mod mint_nfts {

    use anchor_spl::{metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3}, token_interface};

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

    //mint token for itself like mine token
    pub fn mining_token(ctx:Context<MintToken>,amount: u64) -> Result<()> {
        msg!("We will mine the G Coins");
        msg!("Mint account = {:?}",ctx.accounts.mint.key());
        
        let signer_seeds : &[&[&[u8]]] = &[&[b"mint1",&[ctx.bumps.mint]]];

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(), 
            }
        ).with_signer(signer_seeds);

        token_interface::mint_to(cpi_context, amount)?;

        msg!("Mining successfully...");
        Ok(())
    }

    pub fn transfer_token(ctx: Context<TransferToken>,amount: u64) -> Result<()> {
        msg!("Transfer G coin intiated ");

        let decimals = ctx.accounts.mint.decimals;

        let cpi_accounts = token_interface::TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info()
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();

        let cpi_context = CpiContext::new(
            cpi_program,cpi_accounts);

        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        
        msg!("Transafer successfully...");
        Ok(())
    }


    pub fn mint_nfts(ctx: Context<MintNFT>,nft_name: String, nft_symbol: String, nft_uri: String) -> Result<()> {
        
        msg!("Minting the token");

        token_interface::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo { 
                    mint: ctx.accounts.mint_account.to_account_info(), 
                    to: ctx.accounts.associated_token_account.to_account_info(), 
                    authority: ctx.accounts.payer.to_account_info() } 
            ), 
            1
        )?;
        

        msg!("Minting token Successful");
        msg!("Creating Metadata Account");

        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    mint_authority: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info() 
                } 
            ), 
            DataV2 { 
                name: nft_name, 
                symbol: nft_symbol, 
                uri: nft_uri, 
                seller_fee_basis_points: 0, 
                creators: None, 
                collection: None, 
                uses: None }, 
                false, 
                true, 
                None
            )?;

            msg!("Metadata Account created Successfully");
            msg!("Creating Edition Account");

            create_master_edition_v3(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    CreateMasterEditionV3 {
                        edition: ctx.accounts.edition_account.to_account_info(),
                        metadata: ctx.accounts.metadata_account.to_account_info(),
                        mint: ctx.accounts.mint_account.to_account_info(),
                        mint_authority: ctx.accounts.payer.to_account_info(),
                        payer: ctx.accounts.payer.to_account_info(),
                        update_authority: ctx.accounts.payer.to_account_info(),
                        token_program: ctx.accounts.token_program.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info()
                    }
                ),
                None // max supply 
            )?;

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
        seeds = [b"token1"],
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


// mint token to self account kind of mining the tokens
#[derive(Accounts)]
pub struct MintToken<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint1"],
        bump
    )]
    pub mint : InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer= payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub token_account : InterfaceAccount<'info,TokenAccount>,
    pub token_program : Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>
}

#[derive(Accounts)]
pub struct TransferToken<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint1"],
        bump
    )]
    pub mint: InterfaceAccount<'info,Mint>,

    #[account(
        mut,
    )]
    pub sender_token_account : InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        seeds = [b"token1"],
        bump
    )]
    pub receiver_token_account : InterfaceAccount<'info,TokenAccount>,
    pub token_program: Interface<'info,TokenInterface>
}


// create NFT 
#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: Validate Address by deriving PDA
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key()
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Validate Address by deriving PDA
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),mint_account.key().as_ref(),b"edition"],
        bump,
        seeds::program = token_metadata_program 
    )]
    pub edition_account : UncheckedAccount<'info>,

    // Create a mint account
    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer.key(),
        mint::freeze_authority = payer.key()
    )]
    pub mint_account : Account<'info,token::Mint>,

    // Create associated token account
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint_account,
        associated_token::authority = payer,
    )]
    pub associated_token_account :Account<'info,token::TokenAccount>,

    pub token_program: Program<'info,Token>,
    pub token_metadata_program: Program<'info,Metadata>,
    pub associated_token_program: Program<'info,AssociatedToken>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info,Rent>

}