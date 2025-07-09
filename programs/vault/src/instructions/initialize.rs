#![allow(unused_imports)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use crate::{state::VaultState, constants::ANCHOR_DISCRIMINATOR, error::ErrorCode};

#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(
        mut
    )]
    pub user:Signer<'info>,


    #[account(
        init,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + vault_state::INIT_SPACE,
        seeds=[b"state",user.key().as_ref()],
        bumps,
    )]
    pub vault_state:Account<'info,VaultState>,

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault:SystemAccount<'info>,

    pub system_program:Program<'info,System>
}

impl<'info> Initialize<'info>{

    pub fn initialize_vault(&self,bumps:&InitializeBumps)->Result<()>{
        let rent_excempt = Rent::get()?.minimum_balance(self.account.vault.to_account_info().date_len());

        let user_balance  = self.user.lamports();

        require_gte!(
            user_balance,
            rent_excempt,initialize_vault
            ErrorCode::InsufficientFunds
        );

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer{
            from:self.user.to_account_info(),
            to:self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer(cpi_ctx,rent_excempt)?;


        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        
    }
}