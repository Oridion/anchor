use anchor_lang::prelude::*;
use super::*;

///------------------------------------------------------------//
/// COMET PDA
///------------------------------------------------------------//
#[derive(Accounts)]
pub struct CreateComet<'info> {
    #[account(mut)]
    pub universe: Account<'info,Universe>,
    #[account(mut)]
    pub planet: Account<'info,Planet>,
    #[account(mut,address = TREASURY_PUBKEY)]
    pub treasury: SystemAccount<'info>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
}

//Constrain = The "creator public key" being passed during this update
//must match the "creator" field already set in the data of the comet account when initialized
//Try to use this for HOP and DESTINATION RETRIEVAL FUNDS
//Hope to planet will always go from planet to planet
#[derive(Accounts)]
pub struct PlanetHop<'info> {
    #[account(mut)]
    pub to_planet: Account<'info,Planet>,
    #[account(mut)]
    pub from_planet: Account<'info,Planet>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>
}

#[derive(Accounts)]
pub struct WithdrawAccounts<'info> {
    #[account(mut)]
    pub from_planet: Account<'info,Planet>,
    #[account(mut)]
    pub destination: SystemAccount<'info>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>
}

//Star hop from Planet to Split stars
#[derive(Accounts)]
#[instruction(star_one_id: String, star_two_id: String)]
pub struct StarHopTwoStart<'info> {
    #[account(mut)]
    pub from_planet: Account<'info,Planet>,
    #[account(init, payer = manager, space = Star::LEN,
        seeds = [
            STAR_SEED_PRE,
            star_one_id.as_ref(),
            STAR_SEED_POST
        ],
        bump
    )]
    pub star_one: Account<'info, Star>,
    #[account(init, payer = manager, space = Star::LEN,
        seeds = [
            STAR_SEED_PRE,
            star_two_id.as_ref(),
            STAR_SEED_POST
        ],
        bump
    )]
    pub star_two: Account<'info, Star>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

//Return from stars to destination planet
#[derive(Accounts)]
pub struct StarHopTwoEnd<'info> {
    #[account(mut)]
    pub to_planet: Account<'info,Planet>,
    #[account(mut, has_one = manager, constraint = manager.key == &star_one.manager)]
    pub star_one: Account<'info, Star>,
    #[account(mut, has_one = manager, constraint = manager.key == &star_two.manager)]
    pub star_two: Account<'info, Star>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>
}


#[derive(Accounts)]
#[instruction(star_one_id: String, star_two_id: String, star_three_id: String )]
pub struct StarHopThreeStart<'info> {
    #[account(mut)]
    pub from_planet: Account<'info,Planet>,
    #[account(init, payer = manager, space = Star::LEN,
        seeds = [
            STAR_SEED_PRE,
            star_one_id.as_ref(),
            STAR_SEED_POST
        ],
        bump
    )]
    pub star_one: Account<'info, Star>,

    #[account(init, payer = manager, space = Star::LEN,
        seeds = [
            STAR_SEED_PRE,
            star_two_id.as_ref(),
            STAR_SEED_POST
        ],
        bump
    )]
    pub star_two: Account<'info, Star>,

    #[account(init, payer = manager, space = Star::LEN,
        seeds = [
            STAR_SEED_PRE,
            star_three_id.as_ref(),
            STAR_SEED_POST
        ],
        bump
    )]
    pub star_three: Account<'info, Star>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StarHopThreeEnd<'info> {
    #[account(mut)]
    pub to_planet: Account<'info,Planet>,
    #[account(mut, has_one = manager, constraint = manager.key == &star_one.manager)]
    pub star_one: Account<'info, Star>,
    #[account(mut, has_one = manager, constraint = manager.key == &star_two.manager)]
    pub star_two: Account<'info, Star>,
    #[account(mut, has_one = manager, constraint = manager.key == &star_three.manager)]
    pub star_three: Account<'info, Star>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub manager: Signer<'info>
}

#[account]
pub struct Star {
    pub id: String,
    pub amount: u64,
    pub manager: Pubkey
}
impl Star {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + STRING_LENGTH_PREFIX + STAR_ID_LENGTH //Star ID (String)
        + LAMPORT_LENGTH
        + PUBLIC_KEY_LENGTH; // Lamports.
}
