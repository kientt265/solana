use anchor_lang::prelude::*;

declare_id!("8a7TbvmxWb8tzTEp5CQRem98GaTum8cgBjokWxcJK8Xd");

const MAX_NAME_LENGTH: usize = 50;
const MAX_STREET_LENGTH: usize = 100;


#[program]
pub mod account_data {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name_new: String, house_number_new: u8, street_new: String ) -> Result<()> {
        
        msg!("Greetings from: {:?}", ctx.program_id);
        let account_data_new = &mut ctx.accounts.data_info;
        account_data_new.name = name_new;
        account_data_new.house_number = house_number_new;
        account_data_new.street = street_new;

        // *ctx.accounts.data_info = DataInfo{
        //     name_new,
        //     house_number_new,
        //     street_new
        // }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + 1 + 4 + MAX_NAME_LENGTH + 4 + MAX_STREET_LENGTH,

    )]

    data_info: Account<'info, DataInfo>,
    system_program: Program<'info, System>
}


#[account]
pub struct DataInfo{
    pub name: String,
    pub house_number: u8,
    pub street: String
}