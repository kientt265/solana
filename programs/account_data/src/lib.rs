use anchor_lang::prelude::*;

declare_id!("8a7TbvmxWb8tzTEp5CQRem98GaTum8cgBjokWxcJK8Xd");

#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[account]
