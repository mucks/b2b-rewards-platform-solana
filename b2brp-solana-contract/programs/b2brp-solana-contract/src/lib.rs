use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, MintTo, Token},
};

declare_id!("3EMMqhy7w8PYobaxMfxhH6CMjxPgpVmdCZZSq2Pt2oys");

// TODO: improve comments and readability

#[program]
pub mod b2brp_solana_contract {
    // use anchor_lang::solana_program::system_program;
    use anchor_lang::solana_program::{native_token::LAMPORTS_PER_SOL, program::invoke};
    use anchor_lang::system_program;
    use anchor_spl::{
        associated_token,
        token::{initialize_mint, InitializeMint},
    };
    use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v3};

    use super::*;

    pub fn mint_company_license(
        ctx: Context<MintCompanyLicense>,
        metadata_title: String,
        metadata_symbol: String,
        metadata_uri: String,
    ) -> Result<()> {
        msg!("Creating mint account");
        msg!("Mint: {}", &ctx.accounts.mint.key());

        system_program::create_account(
            // Cpi is a helper for cross-program invocations.
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount {
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.token_program.to_account_info(),
                },
            ),
            LAMPORTS_PER_SOL,
            82,
            &ctx.accounts.token_program.key(),
        )?;

        msg!("Intializing mint account");
        msg!("Mint: {}", &ctx.accounts.mint.key());

        initialize_mint(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                InitializeMint {
                    mint: ctx.accounts.token_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key()),
        )?;

        msg!("Creating token account");
        msg!("Token address: {}", &ctx.accounts.token_account.key());

        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.mint_authority.to_account_info(),
                associated_token: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        msg!("Minting tokens to token account");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        msg!("Token address: {}", &ctx.accounts.token_account.key());

        mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Creating metadata account");
        msg!("Metadata account address: {}", &ctx.accounts.metadata.key());

        invoke(
            &create_metadata_accounts_v3(
                ID,
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                metadata_title,
                metadata_symbol,
                metadata_uri,
                None,
                1,
                true,
                false,
                None,
                None,
                None,
            ),
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Creating master edition metadata account");
        msg!(
            "Master edition metadata account address: {}",
            &ctx.accounts.metadata.key()
        );

        invoke(
            &create_master_edition_v3(
                ID,
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint_authority.key(),
                Some(0),
            ),
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        msg!("Token mint process completed successfully");

        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create(ctx: Context<Create>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintCompanyLicense<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Signer<'info>,

    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: We're about to create this with Metaplex
    pub token_metadata_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space=264)]
    pub sp_rewards_container: Account<'info, SpRewardsContainer>,

    #[account(init, payer = user, space=264)]
    pub company_token_supply: Account<'info, CompanyTokenSupply>,

    #[account(init, payer = user, space=264)]
    pub company_license: Account<'info, CompanyLicense>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// ERC-721 contract for the Enterprise License
#[account]
pub struct CompanyLicense {
    pub user: Pubkey,
    pub company_name: String,
    pub company_details: String,
    pub company_logo: String,
    pub rarity: u8,
    pub upper_credit_limit: u64,
}

// ERC-20 contract for the Enterprise Supply of Tokens
#[account]
pub struct CompanyTokenSupply {
    pub user: Pubkey,
    pub amount: u64,
}

// ERC-1155 contract for the Service Providers
#[account]
pub struct SpRewardsContainer {
    pub user: Pubkey,
    pub amount: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Mint failed!")]
    MintFailed,
}

#[derive(Accounts)]
pub struct Initialize {}
