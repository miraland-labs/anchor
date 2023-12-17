#![allow(dead_code)]

use anchor_lang::prelude::borsh::io::Write;
use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Needed to declare accounts.
declare_id!("AnchoL61Nt2sgXvrXYUHxRQgEgaJ4ueMg5xJQVyFJ5Gs");

#[derive(Accounts)]
pub struct GenericsTest<'info, T, U, const N: usize>
where
    T: AccountSerialize + AccountDeserialize + Owner + Clone,
    U: BorshSerialize + BorshDeserialize + Default + Clone,
{
    pub non_generic: AccountInfo<'info>,
    pub generic: Account<'info, T>,

    pub const_generic: AccountLoader<'info, FooAccount<N>>,
    pub const_generic_loader: AccountLoader<'info, FooAccount<N>>,
    pub associated: Account<'info, Associated<U>>,
}

#[account(zero_copy)]
pub struct FooAccount<const N: usize> {
    pub data: WrappedU8Array<N>,
}

#[account]
#[derive(Default)]
pub struct Associated<T>
where
    T: BorshDeserialize + BorshSerialize + Default,
{
    pub data: T,
}

#[derive(Copy, Clone)]
pub struct WrappedU8Array<const N: usize>(u8);
impl<const N: usize> BorshSerialize for WrappedU8Array<N> {
    fn serialize<W: Write>(&self, _writer: &mut W) -> borsh::io::Result<()> {
        todo!()
    }
}
impl<const N: usize> BorshDeserialize for WrappedU8Array<N> {
    // fn deserialize(_buf: &mut &[u8]) -> borsh::io::Result<Self> {
    //     todo!()
    // }

    // MI, for borsh 0.10.3 and above
    fn deserialize_reader<R: borsh::io::Read>(_reader: &mut R) -> borsh::io::Result<Self> {
        todo!()
    }
}
impl<const N: usize> Owner for WrappedU8Array<N> {
    fn owner() -> Pubkey {
        crate::ID
    }
}
