use anchor_lang::prelude::*;

declare_id!("9g8n5dxYZSXchPTKXvRcHRCTzHC4n5zNYjnSo7fdhKeK");

#[program]
pub mod points_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
