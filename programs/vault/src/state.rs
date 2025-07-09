use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState{
    vault_bump:u8,
    state_bump:u8
}