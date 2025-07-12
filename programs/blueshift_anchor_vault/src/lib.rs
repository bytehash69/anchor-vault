use anchor_lang::prelude::*;

declare_id!("DVyeivuEkP3tynMx8kH6vJyEAtXGV2H1BRh6MA1S8jwi");

#[program]
pub mod blueshift_anchor_vault {
    use anchor_lang::system_program::{transfer, Transfer};

    use super::*;


    pub fn deposit(ctx:Context<VaultAction>, amount: u64) -> Result<()> {

        // Checks if the vault account alreadly exists
        require_eq!(ctx.accounts.vault.lamports(), 0, VaultError::VaultAlreadyExists);
        // Checks if the amount being depositing is greater than the minimum balance require to create the vault account
        require_gt!(amount, Rent::get()?.minimum_balance(0), VaultError::InvalidAmount);
        
        // Getting all the accounts required
        let signer = ctx.accounts.signer.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        
        // Creating the cpi context required for the transfer function 
        let cpi_context = CpiContext::new(
            system_program,
            // Transfer amount from signer to vault 
            Transfer{from: signer,to: vault}
        );
        
        // CPI into the system program for the transfer function
        transfer(cpi_context, amount)?;
        
        Ok(())
    }
    
    pub fn withdraw(ctx:Context<VaultAction>, amount: u64) -> Result<()> {

        // Checks if there is some amount in the vault
        require_neq!(ctx.accounts.vault.lamports(), 0, VaultError::InvalidAmount);

        // Getting all the accounts required
        let signer = ctx.accounts.signer.to_account_info();
        let vault = ctx.accounts.vault.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        
        // Creating the seeds for vault pda
        let signer_key = signer.key();
        let signer_seeds: &[&[u8]] = &[b"vault",signer_key.as_ref(),&[ctx.bumps.vault]];
        let seeds = &[signer_seeds];

        // Creating the cpi context required for the transfer function 
        // and passing the seeds to derive the vault
        let cpi_context = CpiContext::new_with_signer(
            system_program,
            // Transfer amount from vault to signer 
            Transfer{from: vault,to: signer},
            // The seeds
            seeds
        );

        transfer(cpi_context, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    // The signer who wants to deposit and pay for the transactions
    #[account(mut)]
    pub signer: Signer<'info>,

    // A system account pda for vault
    #[account(
        mut,
        seeds = [b"vault",signer.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // System program for creating the pda
    pub system_program: Program<'info,System>
}

#[error_code]
pub enum VaultError {
  #[msg("Vault already exists")]
  VaultAlreadyExists,
  #[msg("Invalid amount")]
  InvalidAmount,
}