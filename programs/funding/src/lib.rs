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
   
}


//also called 'macro'
#[derive(Accounts)]
pub struct Create<'info>
{
    #[account(init, payer=user,space=9000)]
    pub campaign: Account<'info, Campaign>,
    //user who calling create function
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

