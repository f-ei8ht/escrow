pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AEkEcM7ANVVcmUXLyMKMw89XRfYGz71ZYJJLfziqcN3v");

#[program]
pub mod escrow {
    use super::*;

    pub fn make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_offered_amount: u64,
        token_b_wanted_amount: u64,
    ) -> Result<()> {
        make_offer::handler(context, id, token_a_offered_amount, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        take_offer::take_offer_handler(context)
    }

    pub fn refund_offer(context: Context<RefundOffer>) -> Result<()> {
        refund::refund_offer(context)
    }
}
