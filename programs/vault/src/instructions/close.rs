#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::{state::VaultState, constants::ANCHOR_DISCRIMINATOR, error::ErrorCode};

#[derive(ACcounts)]
pub struct Close<'info>{

    #[account]
    pub user:Signer<'info>,

    #[account(
        close = user,
        seeds=[b"vault",user.key().as_ref()],
        bump:vault_state.vault_bump
    )]
    pub vault_state:Account<'info,VaultState>,

    #[account(
        close = user,
        seeds=[b"state"],user.key().as_ref(),
        bump:vault_state.state_bump
    )]
    pub vault_state:Account<'info,VaultState>,

    pub system_program:Program<'info,System>
}

impl<'info> Close<'info>{
    pub fn close(&self)->Result<()>{
        let vault_balance = self.vault.lamports();

        if(vault_balance>0){
            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts  = Transfer{
                from:self.vault.to_account_info(),
                to:self.user.to_account
            };

            let seeds = &[
                b"vault",
                self.vault_state.key().as_ref(),
                &[self.vault_state.vault_bump]
            ];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,seeds);
            transfer(cpi_ctx,vault_balance)?
        }

        Ok(())
    }
}