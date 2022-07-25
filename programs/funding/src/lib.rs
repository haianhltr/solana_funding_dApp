use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("A6ckA4ZaCUTmDvRYVseycUx2dCTfPMoAMoDSj1FFi9qH");

#[program]
pub mod funding {
    use super::*;

    // create a campaign, only owner of this campaign can withdraw the fund
    pub fn create(ctx: Context<Create>, name: String, desc: String) -> ProgramResult {
        let campaign  = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.desc = desc;
        campaign.amount_donated = 0;
        //admin is the one that can withdraw the money
        campaign.admin = *ctx.accounts.user.key;
        Ok(())
    }
   

    //create a withdraw function
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;
        if campaign.admin != *user.key
        {
            return Err(ProgramError::IncorrectProgramId);
        }

        //calc rent balance
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        //check if campaign acc have enough fund to withdraw
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount 
        {
            return Err(ProgramError::InsufficientFunds);
        }

        //transfer fund
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())

    }


    //create donate function
    pub fn donate(ctx: Context<Donate>, amount: u64) -> ProgramResult{

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info()
            ]
        );
        (&mut ctx.accounts.campaign).amount_donated += amount;
        Ok(())
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

#[derive(Accounts)]
pub struct Withdraw<'info>{
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Donate<'info>{
    pub campaign: Account<'info, Campaign>,
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
