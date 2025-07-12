#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

pub mod instructions;
pub use instructions::*;

pub mod constants;
pub use constants::*;


declare_id!("9tQEE84CEPmudCtPj71BuvCdDwKPTgjLthMcpkNxNv5D");

#[program]
pub mod vault {
    use super::*;
    pub fn initialize(ctx:Context<Initialize>)->Result<()>{
        ctx.accounts.initialize_vault(&ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx:Context<Payment>,amount:u64)->Result<()>{
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    pub fn close_vault(ctx:Context<Close>)->Result<()>{
        ctx.accounts.close_vault()?;
        Ok(())
    }
  
}

