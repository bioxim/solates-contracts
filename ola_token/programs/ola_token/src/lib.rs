use anchor_lang::prelude::*;

declare_id!("RQRM8HbKk2YrsPYJfstyJLUmJYUumyPwFykTzj1fZuw");

#[program]
pub mod ola_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
