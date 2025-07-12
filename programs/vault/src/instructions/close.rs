use anchor_lang::{
    prelude::*,
    system_program::{Transfer,transfer}
};

use crate::{state::*,errors::*,state::*};

#[derive(Accounts)]
pub struct Close<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(
        mut,
        seeds=[b"state",signer.key().as_ref()],
        bump = vault_state.state_bump
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

impl<'info> Close<'info>{
    pub fn close_vault(&mut self)-> Result<()>{
        let vault_balance = self.vault.lamports();

        if vault_balance>0{
            let cpi_program = self.system_program.to_account_info();

            let vault_state_key = self.vault_state.key();
            let seeds = [
                b"vault",
                vault_state_key.as_ref(),
                &[self.vault_state.vault_bump]
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_accounts = Transfer{
                from:self.vault.to_account_info(),
                to:self.signer.to_account_info(),
            
            };

            let cpi_context = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
            transfer(cpi_context,vault_balance)?;
        }

        Ok(())
    }
}