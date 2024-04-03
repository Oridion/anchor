use anchor_lang::prelude::*;
use super::*;

///------------------------------------------------------------//
/// DEPOSIT PDA
/// Created by manager
/// Tracks deposit info.
//  Manager updates on each hop
/// Should be deleted on close
///------------------------------------------------------------//
#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateDeposit<'info> {
    #[account(
        init,
        payer = manager,
        space = Deposit::LEN,
        seeds = [
            DEPOSIT_SEED_PRE,
            id.as_ref(),
            DEPOSIT_SEED_POST,
        ],
        bump
    )]
    pub deposit: Account<'info,Deposit>,
    #[account(mut)]
    pub universe: Account<'info,Universe>,
    #[account(mut)]
    pub creator: SystemAccount<'info>,
    #[account(mut)]
    pub manager: Signer<'info>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateDeposit<'info> {
    #[account(mut)]
    pub comet: Account<'info, Comet>,
    #[account(mut, address = MANAGER_PUBKEY)]
    pub creator: Signer<'info>
}

#[derive(Accounts)]
pub struct CloseDeposit<'info> {
    #[account(mut, has_one = manager, constraint = manager.key == &deposit.manager)]
    pub deposit: Account<'info,Deposit>,
    #[account(mut)]
    pub manager: Signer<'info>
}



//------------------------------------------------------------//
// DEPOSIT ACCOUNT
//------------------------------------------------------------//
/// Users creates comet by transferring funds to the universe.
/// - funds "hops" from universe to stars/planets until final destination wallet.
/// - During comet life span, user responsible for "cranking" comet to final destination
/// - User can track progress / location.
/// - Comets will be deleted with no trace after completing their path.
/// - One wallet = One comet.
/// - star name must be 6 chars long.
/// - each string location set variable must a certain length.
#[account]
pub struct Deposit {
    pub id: String, //Crank ID - used to identify and crank the comet trajectory
    pub creator: Pubkey,
    pub manager: Pubkey,
    pub created: i64,
    pub pda: Pubkey,
    pub bump: u8,
    pub deposit: u64, //must be lamports
    pub loc: String, //Current location.
    pub hops: u8, //How many hops comet has made
    pub up: i64, //Last updated timestamp
    pub hpfe: u64, //Fee
    pub hsfe2: u64, //Fee
    pub hsfe3: u64, //Fee
    pub wfe: u64, //Fee
}
impl Deposit {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + COMET_ID_LENGTH //Crank id
        + PUBLIC_KEY_LENGTH //creator
        + PUBLIC_KEY_LENGTH //Manager
        + TIMESTAMP_LENGTH // Created timestamp
        + PUBLIC_KEY_LENGTH // PDA
        + U8_LENGTH //Bump
        + LAMPORT_LENGTH //Deposit
        + COMET_LOCATION // Planet Location
        + U8_LENGTH //Hops made
        + TIMESTAMP_LENGTH // Last updated
        + U64_LENGTH //Hop planet Fee (lamports)
        + U64_LENGTH //Hop star2 Fee (lamports)
        + U64_LENGTH //Hop star3 Fee (lamports)
        + U64_LENGTH; //Withdraw Fee (lamports)
}
