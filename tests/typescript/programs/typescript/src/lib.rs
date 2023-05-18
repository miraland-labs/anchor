//! The typescript example serves to show how one would setup an Anchor
//! workspace with TypeScript tests and migrations.

use anchor_lang::prelude::*;

declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[program]
pub mod typescript {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
