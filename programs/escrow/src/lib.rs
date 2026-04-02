use anchor_lang::prelude::*;
mod make_offer;

use  crate::{make_offer::*}

declare_id!("6R6uUFxA8UbWdFmzVSRRAZfkBfE79qovCEioCmzgw62t");

#[program]
pub mod escrow_program {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        Ok(())
    }
}

