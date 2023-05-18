use anchor_lang::prelude::*;

declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[program]
mod sysvars {
    use super::*;
    pub fn sysvars(_ctx: Context<Sysvars>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Sysvars<'info> {
    pub clock: Sysvar<'info, Clock>,
    pub rent: Sysvar<'info, Rent>,
    pub stake_history: Sysvar<'info, StakeHistory>,
}
