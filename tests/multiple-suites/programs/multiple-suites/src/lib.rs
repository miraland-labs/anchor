use anchor_lang::prelude::*;

declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[program]
pub mod multiple_suites {
    use super::*;

    // _val to ensure tx are different so they don't get rejected.
    pub fn initialize(_ctx: Context<Initialize>, _val: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
