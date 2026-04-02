use anchor_lang::prelude::*;

#[account]
#[derive(Accounts)]
pub struct MakeOffer{
    //used to manage associated token accounts
    //where the wallet hold the token
    pub associated_token_program:Program<'info, AssociatedToken>,

    //work  with the token or new extension token
    pub token_program: Interface<'info, TokenInterface>,

    //used to create accounts
    pub system_program:Program<'info, System>,

    //maker
    #[account(mut)]
    pub maker:Signer<'info>,

    #[account(mint::token_program = token_program)]   
    pub token_mint_a:InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]   
    pub token_mint_b :InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        payer = maker,
        space = Offer::DISCRIMENATOR.len() + Offer::INIT_SPACE,
        seeds = 
    )]
    pub Offer:<>

}


pub fn make_offer(ctx:Context<MakeOffer>) -> Result<()>{
    Ok(())
}