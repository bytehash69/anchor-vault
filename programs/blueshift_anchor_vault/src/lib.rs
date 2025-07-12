use anchor_lang::prelude::*;

declare_id!("DVyeivuEkP3tynMx8kH6vJyEAtXGV2H1BRh6MA1S8jwi");

#[program]
pub mod blueshift_anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
