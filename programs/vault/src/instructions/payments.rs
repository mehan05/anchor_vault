#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::{state::VaultState, constants::ANCHOR_DISCRIMINATOR, error::ErrorCode};

#[derive(Accounts)]
pub struct Payments<'info>{
    #[account(mut)]
    pub user:Signer<'info>,


    #[account(
        seeds=[b"state",user.key().as_ref()],
        bump:vault_state.vault_bump
    )]
    pub vault_state:Account<'info,VaultState>,


    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump:vault_state.vault_bump        
    )]
    pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>
}

impl<'info> Payments<'info>{

    pub fn deposit(&self, amount:u64)->Result<()>{

        require_gt(amount,0,ErrorCode::InsufficientFunds);

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_ctx,amount)?;
    }

    pub fn withdraw(&self,amount:u64)->Result<()>{

        require_gt(amount,0,ErrorCode::InsufficientFunds);

        let rent_excempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());

        let vault_balance = self.vault.lamports();

        require(amount<= vault_balance-rent_excempt,ErrorCode::InsufficientFunds);

        require_gte!(
            vault_balance,
            rent_excempt,
            ErrorCode::InsufficientFunds
        );


        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.vault.to_account_info(),
            to:self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.key().as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&[seeds[..]]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts
        ,[signer_seeds]
        );

        transfer(cpi_ctx,amount)?;


    }
}