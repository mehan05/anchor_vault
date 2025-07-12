use anchor_lang::{
    prelude::*,
    system_program::{Transfer,transfer}
};

use crate::{constants::*, state::*, errors::*};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,
    
    #[account(
        init,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + VaultState::INIT_SPACE,
        seeds=[b"state",signer.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info,VaultState>,

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>,
}

impl<'info> Initialize<'info>{
     pub  fn initialize_vault(&mut self,bumps:&InitializeBumps)->Result<()>{

        let rent = Rent::get()?.minimum_balance(self.vault_state.to_account_info().data_len());

        let user_balance = self.signer.lamports();

        require!(
            user_balance >= rent,
            VaultError::InsufficientUserBalance
        );

        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer{
            from:self.signer.to_account_info(),
            to:self.vault.to_account_info(),   
        };

        let cpi_context = CpiContext::new(cpi_program,cpi_accounts);

        transfer(cpi_context,rent)?;

        self.vault_state.state_bump = bumps.vault_state;
        self.vault_state.vault_bump = bumps.vault;

        Ok(())

    }
}