use anchor_lang::prelude::*;

declare_id!("7twKMSNdrhAoyCRihonwr7gLkeFeM2giG1NiQLiev6Bs");

#[program]
pub mod ola_economy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
