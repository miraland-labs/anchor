use anchor_lang::prelude::*;

declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[program]
pub mod unchecked_account {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    unchecked: UncheckedAccount<'info>,
}
