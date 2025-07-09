#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
mod state;

use anchor_lang::prelude::*;

use instruction::*;

declare_id!("9tQEE84CEPmudCtPj71BuvCdDwKPTgjLthMcpkNxNv5D");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_vault(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Payments>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Payments>, amount: u64) -> Result<()>{
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
