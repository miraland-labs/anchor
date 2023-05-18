use anchor_lang::prelude::*;

declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[program]
mod system_accounts {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub authority: Signer<'info>,
    pub wallet: SystemAccount<'info>,
}
