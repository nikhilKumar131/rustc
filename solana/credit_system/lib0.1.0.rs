use anchor_lang::prelude::*;

declare_id!("UVssmLSQUP3guoEKcuGLdUdkW2Mezn24DLjeguJLdd3");

#[program]
pub mod game {
    use super::*;
    // handler function
    pub fn initialize_account(ctx: Context<InitializeAccount>, credit: u64) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;

        ctx.accounts.credits.total_credit += credit;

        user_stats.credit = credit;
        user_stats.bump = *ctx.bumps.get("user_stats").unwrap();
        Ok(())
    }

    pub fn add_credit(ctx: Context<IndCredit>, new_level: u64) -> Result<()> {
        ctx.accounts.user_stats.credit += new_level;
        ctx.accounts.credits.total_credit += new_level;
        Ok(())
    }

    pub fn remove_credit(ctx: Context<IndCredit>, new_level: u64) -> Result<()> {
        ctx.accounts.user_stats.credit -= new_level;
        ctx.accounts.credits.total_credit -= new_level;
        Ok(())
    }

    pub fn get_total_credits(ctx: Context<IndCredit>) -> Result<u64> {
        Ok(ctx.accounts.credits.total_credit)
    }

    pub fn get_your_credits(ctx: Context<IndCredit>) -> Result<u64> {
        let user_stats = &mut ctx.accounts.user_stats;
        let total_credits = user_stats.credit;
        Ok(total_credits)
    }

}

#[account]
pub struct IndividualCredit {
    credit: u64,
    bump: u8,
}

#[account]
pub struct Credit {
    total_credit: u64,
}

// validation struct
#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // space: 8 discriminator + 2 level + 4 name length + 200 name + 1 bump
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1, seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    pub user_stats: Account<'info, IndividualCredit>,
    #[account(mut)]
    pub credits: Account<'info, Credit>,
    pub system_program: Program<'info, System>,
}

// validation struct
#[derive(Accounts)]
pub struct IndCredit<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, IndividualCredit>,
    #[account(mut)]
    pub credits: Account<'info, Credit>,
}
