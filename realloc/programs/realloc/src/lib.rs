use anchor_lang::prelude::*;

declare_id!("6es3zYwjvDC4qCQFQRxykZN9ZTJVKy6B93efny6CGwkg");

#[program]
pub mod realloc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        let message_account = &mut ctx.accounts.message_account;
        message_account.message = input;
        Ok(())
    }
    pub fn update(ctx: Context<UpdateRelocation>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = Message::required_space(input.len()),
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct UpdateRelocation<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        realloc = Message::required_space(input.len()),
        realloc::payer = user,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct  Message {
    pub message: String
}

impl Message {
    pub fn required_space(input_len: usize) -> usize{
        8 + 4 +input_len
    }
}