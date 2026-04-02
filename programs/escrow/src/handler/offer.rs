use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] //calculate the spaceneeded  for this account
pub struct Offer {
    //identifier of the offer
    pub id: u32,
    // who made the offer
    pub maker: PubKey,
    //token mint of the token wanted
    pub token_mint_a: PubKey,
    //token mint of the token being wanted
    pub token_mint_b: PubKey,
    //token mint of the b token wanted
    pub token_b_wanted_amount: u32,
    //Used to calculate the address of this account
    pub bump: u32,
}
