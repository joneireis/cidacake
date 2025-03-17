use anchor_lang::prelude::*;

declare_id!("HYvvpb1skqJFw6LT84DaTwmGo1BjA9YrDW7aqgLUwt6Y");

#[program]
pub mod cidacake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
