use anchor_lang::prelude::*;

declare_id!("CrgoZFjCU9ULYvthH1k2ibrDhGczXozWBMCiDT9w6JEN");

#[program]
pub mod lottery {
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
