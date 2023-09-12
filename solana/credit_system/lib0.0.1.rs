use anchor_lang::prelude::*;

declare_id!("CSJ7k6FP1ngVBN371aUbUgYwymZ3tsHCEd9bzV7rPmn8");

#[program]
mod credit_system {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, initial_balance: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.total_balance = initial_balance;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let user_account = &mut ctx.accounts.user_account;

        // Ensure the deposit amount is greater than zero
        if amount == 0 {
            return Err(CreditError::InvalidDepositAmount.into());
        }

        // Transfer credits from the user's token account to the data account
        if user_account.balance < amount {
            return Err(CreditError::InsufficientBalance.into());
        }

        user_account.balance -= amount;
        data_account.total_balance += amount;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        let user_account = &mut ctx.accounts.user_account;

        // Ensure the withdrawal amount is greater than zero
        if amount == 0 {
            return Err(CreditError::InvalidWithdrawalAmount.into());
        }

        // Ensure the user has sufficient credits to withdraw
        if data_account.total_balance < amount {
            return Err(CreditError::InsufficientCredits.into());
        }

        // Transfer credits from the data account to the user's token account
        user_account.balance += amount;
        data_account.total_balance -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 64 + 8)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub data_account: Account<'info, DataAccount>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct DataAccount {
    pub total_balance: u64,
}

#[account]
pub struct TokenAccount {
    pub owner: Pubkey, // Address of the token account owner
    pub balance: u64,  // Token balance
}

#[error_code]
pub enum CreditError {
    #[msg("Invalid deposit amount")]
    InvalidDepositAmount,
    #[msg("Insufficient balance for deposit")]
    InsufficientBalance,
    #[msg("Invalid withdrawal amount")]
    InvalidWithdrawalAmount,
    #[msg("Insufficient credits for withdrawal")]
    InsufficientCredits,
}
