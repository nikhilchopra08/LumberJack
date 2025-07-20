use anchor_lang::prelude::*;

declare_id!("BQWnD1XaFoHN1jM1ACA5rhnYipPCvYayxs9JpKNRfirD");

#[program]
pub mod lumberjack {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
