use anchor_lang::prelude::*;

declare_id!("HK3vV5PNkQqXgRbMSzQRGNUvYU6RKUc2dj8LMe1mYtSA");

#[program]
pub mod game {
    use super::*;
    // handler function
    pub fn initialize_account(ctx: Context<InitializeAccount>, credit: u64) -> Result<()> {
        let user_stats = &mut ctx.accounts.user_stats;
        let credits = &mut ctx.accounts.credits;

        credits.credit_score = credit;

        user_stats.credit = credit;
        user_stats.bump = *ctx.bumps.get("user_stats").unwrap();
        credits.bump = *ctx.bumps.get("credits").unwrap();

        Ok(())
    }

    pub fn borrow_credit(ctx: Context<IndCredit>, new_level: u64) -> Result<()> {
        let limit = ctx.accounts.credits.credit_score;

        if limit - ctx.accounts.user_stats.credit <= new_level {
            ctx.accounts.user_stats.credit += new_level;
        } else {
            return Err(MyError::E1.into());
        }

        Ok(())
    }

    pub fn return_credit(ctx: Context<IndCredit>, new_level: u64) -> Result<()> {
        if ctx.accounts.user_stats.credit >= new_level {
            ctx.accounts.user_stats.credit -= new_level;
        } else {
            return Err(MyError::E2.into());
        }

        Ok(())
    }

    pub fn get_credit_limit(ctx: Context<IndCredit>) -> Result<u64> {
        Ok(ctx.accounts.credits.credit_score)
    }

    pub fn get_borrowed_credit(ctx: Context<IndCredit>) -> Result<u64> {
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
    credit_score: u64,
    bump: u8,
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
    #[account(
        init,
        payer = user,
        space = 8 + 2 + 4 + 200 + 1, seeds = [b"user-stats", user.key().as_ref()], bump
    )]
    pub credits: Account<'info, Credit>,
    pub system_program: Program<'info, System>,
}

// validation struct
#[derive(Accounts)]
pub struct IndCredit<'info> {
    pub user: Signer<'info>,
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, IndividualCredit>,
    #[account(mut, seeds = [b"user-stats", user.key().as_ref()], bump = user_stats.bump)]
    pub credits: Account<'info, Credit>,
}

#[error_code]
pub enum MyError {
    #[msg("You can not cross the credit Score limit while borrowing")]
    E1,
    #[msg("can not return extra more credits than borrowed")]
    E2,
}

