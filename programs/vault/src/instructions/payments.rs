use anchor_lang::{
    prelude::*,
    system_program::{Transfer,transfer}
};

use crate::{state::*,errors::*,state::*};

#[derive(Accounts)]
pub struct Payment<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,


    #[account(
        mut,
        seeds = [b"state",signer.key().as_ref()],
        bump  = vault_state.state_bump
    )]
    pub vault_state:Account<'info,VaultState>,


    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump = vault_state.vault_bump
        )]
        pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>
}

impl<'info> Payment<'info>{
    pub fn deposit(&mut self,amount:u64)->Result<()>{
        
        require!(
            amount > 0,
            VaultError::InvalidAmount
        );

        let user_balance = self.signer.lamports();

        require!(
            user_balance >= amount,
            VaultError::InsufficientUserBalance
        );

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from:self.signer.to_account_info(),
            to:self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_context,amount)?;

        Ok(())
    
    }

    pub fn withdraw(&mut self,amount:u64)->Result<()>{
        require!(
            amount > 0,
            VaultError::InvalidAmount
        );

        let vault_balance = self.vault.lamports();

        require!(
            vault_balance>=amount,
            VaultError::InsufficientFunds
        );

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from:self.vault.to_account_info(),
            to:self.signer.to_account_info(),
        };


        let vault_state_key = self.vault_state.key();
        let seeds = [
            b"vault",
            vault_state_key.as_ref(),
            &[self.vault_state.vault_bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

        transfer(cpi_context,amount)?;

        Ok(())

        
    }

}
