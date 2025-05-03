use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites() -> Result<()> {
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,
    #[max_len(50)]
    pub colour: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // Rust lifetime

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump,
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct SetFavourites<'info> {
//     #[account(mut)]
//     pub user: Signer<'info>, // Rust lifetime
//     pub favourites: Account<'info, Favourites>,
// }
