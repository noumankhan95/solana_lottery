use anchor_lang::prelude::*;
use anchor_spl::metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3};
use anchor_spl::metadata::{Metadata, MetadataAccount};
use anchor_spl::{
    associated_token::spl_associated_token_account::solana_program::nonce::state::Data,
    metadata::mpl_token_metadata::types::{CollectionDetails, Creator, DataV2},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};
declare_id!("CrgoZFjCU9ULYvthH1k2ibrDhGczXozWBMCiDT9w6JEN");

#[constant]
pub const NAME: &str = "Token Lottery Ticket #";
#[constant]
pub const SYMBOL: &str = "TLT";
#[constant]
pub const URI: &str = "https://dummyimg.com/";
#[program]
pub mod lottery {

    use anchor_spl::metadata::{
        create_master_edition_v3, sign_metadata, CreateMasterEditionV3, SignMetadata,
    };

    use super::*;

    pub fn initialize_config(
        ctx: Context<Initialize>,
        end_time: i64,
        start_time: i64,
        ticket_price: u64,
    ) -> Result<()> {
        *ctx.accounts.lottery_account = LotteryAccount {
            authority: *ctx.accounts.signer.key,
            bump: ctx.bumps.lottery_account,
            end_time,
            start_time,
            lottery_pot_amount: 0,
            ticket_price,
            total_tickets: 0,
            randomness_account: Pubkey::default(),
            winner_claimed: false,
            winner: Pubkey::default(),
        };
        Ok(())
    }
    pub fn initialize_lottery(ctx: Context<InitializeLottery>) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"collection_mint".as_ref(), &[ctx.bumps.collection_mint]]];
        msg!("Creating Mint Account");
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    to: ctx.accounts.collection_token_account.to_account_info(),
                    authority: ctx.accounts.collection_mint.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;
        msg!("Creating Metadata Account");
        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                },
                &signer_seeds,
            ),
            DataV2 {
                name: NAME.to_string(),
                symbol: SYMBOL.to_string(),
                uri: URI.to_string(),
                seller_fee_basis_points: 0,
                creators: Some(vec![Creator {
                    address: ctx.accounts.collection_mint.key(),
                    verified: false,
                    share: 100,
                }]),
                collection: None,
                uses: None,
            },
            true,
            true,
            Some(CollectionDetails::V1 { size: 0 }),
        )?;

        msg!("Creating Master Edition V3");
        create_master_edition_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMasterEditionV3 {
                    metadata: ctx.accounts.metadata.to_account_info(),
                    mint: ctx.accounts.collection_mint.to_account_info(),
                    mint_authority: ctx.accounts.collection_mint.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                    edition: ctx.accounts.master_edition.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                },
                &signer_seeds,
            ),
            Some(0),
        )?;

        msg!("Verifying Collection");
        sign_metadata(CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            SignMetadata {
                creator: ctx.accounts.collection_mint.to_account_info(),
                metadata: ctx.accounts.metadata.to_account_info(),
            },
            signer_seeds,
        ))?;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let clock = Clock::get()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut,seeds=[b"token_lottery".as_ref()],bump=token_lottery.bump)]
    pub token_lottery: Account<'info, TokenLottery>,

    #[account(init,payer=payer,seeds[token_lottery.total_tickets.to_le_bytes()],bump,mint::decimals=0,mint::authority=collection_mint,mint::freeze_authority=collection_mint,mint::token_program=token_program)]
    #[account(mut,seeds=[b"collection_mint".as_ref()],bump=)]
    pub collection_mint_account: InterfaceAccount<'info, Mint>,
    pub ticket_mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,

    #[account(mut,
    seeds=[b"metadata",token_metadata_program.key().as_ref(),ticket_mint.key().as_ref()],
    bump,
    seeds::program=token_metadata_program.key()
    )]
    pub ticket_metadata: UncheckedAccount<'info>,
    #[account(mut,
    seeds=[b"metadata",token_metadata_program.key().as_ref(),ticket_mint.key().as_ref(),b"edition"],
    bump,
    seeds::program=token_metadata_program.key()
    )]
    pub ticket_master_edition: UncheckedAccount<'info>,
    pub token_metadata_program: Program<'info, Metadata>,

    #[account(init,payer=payer,associated_token::mint=ticket_mint,associated_token::authority=payer,associated_token::token_program=token_program)]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct InitializeLottery<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(init,payer=payer,mint::decimals=0,mint::authority=payer,mint::freeze_authority=payer,seeds=[b"collection_mint".as_ref()],bump)]
    pub collection_mint: InterfaceAccount<'info, Mint>,
    #[account(init,payer=payer,token::mint=collection_mint,token::authority=collection_token_account,
    seeds=[b"collection_associated_token".as_ref()],bump
    )]
    pub collection_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut,
    seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref()],
    bump,
    seeds::program=token_metadata_program.key()
    )]
    pub metadata: UncheckedAccount<'info>,
    #[account(mut,
    seeds=[b"metadata",token_metadata_program.key().as_ref(),collection_mint.key().as_ref(),b"edition"],
    bump,
    seeds::program=token_metadata_program.key()
    )]
    pub master_edition: UncheckedAccount<'info>,

    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init,payer=signer,seeds=[b"lottery"],bump,space=8+LotteryAccount::INIT_SPACE)]
    pub lottery_account: Account<'info, LotteryAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct LotteryAccount {
    pub start_time: i64,
    pub end_time: i64,
    pub total_tickets: i64,
    pub bump: u8,
    pub winner: Pubkey,
    pub winner_claimed: bool,
    pub ticket_price: u64,
    pub authority: Pubkey,
    pub randomness_account: Pubkey,
    pub lottery_pot_amount: i64,
}
