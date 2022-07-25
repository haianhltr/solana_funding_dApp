use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod funding {
    use super::*;

    // create a campaign, only owner of this campaign can withdraw the fund
    pub fn create(ctx: Context<Create>, name: String, desc: String) -> Result<()> {
        let campaign  = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.desc = desc;
        campaign.amount_donated = 0;
        //admin is the one that can withdraw the money
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }
   

    //create a withdraw function
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        if campaign.admin != *user.key
        {
            return Err(ProgramError::IncorrectProgramId);
        }
        //check if campaign acc have enough fund to withdraw
        if **campaign.to_account_info().lamports.borrow() < amount 
        {
            return Err(ProgramError::InsufficientFunds);
        }
    }
}


//also called 'macro'
#[derive(Accounts)]
pub struct Create<'info>
{
    //solana will use a hash function to determine address for a new PDA account based on seeds, nump
    #[account(init, payer=user,space=9000, seeds = [b"CAMPAIGN_DEMO".as_ref(), user.key().as_ref()], bump)]
    pub campaign: Account<'info, Campaign>,
    //user who calling create function
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

//specify what campaign account looks like
#[account]
pub struct Campaign{
    pub admin: Pubkey,
    pub name: String,
    pub desc: String,
    pub amount_donated: u64
}
