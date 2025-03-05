use anchor_lang::prelude::*;
use lever;

// Thêm import cho Lever struct
use lever::program::Lever;
use lever::PowerStatus;

declare_id!("E64FVeubGC4NPNF2UBJYX4AkrVowf74fRJD9q6YhwstN");

#[program]
pub mod hand {
    use super::*;

    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> Result<()> {
        // Sử dụng cấu trúc accounts từ CPI
        lever::cpi::switch_power(
            CpiContext::new(
                ctx.accounts.lever_program.to_account_info(),
                lever::cpi::accounts::SetPowerStatus {
                    power: ctx.accounts.power.to_account_info(),
                }
            ),
            name
        )
    }
}

#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_program: Program<'info, Lever>,  // Bây giờ có thể sử dụng Lever trực tiếp
}
