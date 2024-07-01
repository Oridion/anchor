use anchor_lang::prelude::*;
use super::*;


///-------------------------------------------------------------//
/// PLANETS PDA
/// SEED - name to bytes
/// https://solana.stackexchange.com/questions/2840/best-ways-to-store-long-string-as-pda-seed
/// https://github.com/0xShuk/String-as-PDA/blob/main/lib.rs
///
/// NINE PLANETS
/// - ORNOPU
/// - DORAZ
/// - ANDORA
/// - JEBU
/// - BABALO
/// - KOYOPA
/// - VREDIA
/// - BURU
/// - ZENOS
/// - ISOMA
/// ["ORNOPU","DORAZ","ANDORA","JEBU","BABALO","KOYOPA","VREDIA","BURU","ZENOS","ISOMA"]
///-------------------------------------------------------------//
#[derive(Accounts)]
#[instruction(name: String)]
pub struct CreatePlanet<'info> {
    #[account(
        init,
        payer = creator,
        space = Planet::LEN,
        seeds = [
            PLANET_PDA_SEED_PRE,
            name.as_ref(),
            PLANET_PDA_SEED_POST
        ],
        bump
    )]
    pub planet: Account<'info, Planet>,
    #[account(mut)]
    pub universe: Account<'info, Universe>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}


//1593840 is the minimum amount needed for planets to exist
#[derive(Accounts)]
pub struct DeletePlanet<'info> {
    #[account(mut, close = creator, constraint = &planet.to_account_info().lamports() <= &1593840)]
    pub planet: Account<'info, Planet>,
    #[account(mut)]
    pub universe: Account<'info, Universe>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub creator: Signer<'info>,
}

#[account]
pub struct Planet {
    pub name: String, //Name/ID of planet
    pub pda: Pubkey, //PDA
    pub created: i64, //Planet started
    pub bump: u8, // Bump
    pub visits: u64, //Visitors 
}
impl Planet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + STRING_LENGTH_PREFIX + PLANET_NAME //Planet ID / Name
        + PUBLIC_KEY_LENGTH //PDA
        + TIMESTAMP_LENGTH // Created
        + U8_LENGTH //Bump
        + U64_LENGTH; //Visitors
}