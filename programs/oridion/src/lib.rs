mod variables;
mod errors;
mod accounts_universe;
mod accounts_comet;
mod accounts_planet;
mod shared;

use anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use variables::*;
use shared::*;
use accounts_universe::*;
use accounts_comet::*;
use accounts_planet::*;
use std::io::{Cursor, Write};
use std::ops::DerefMut;


declare_id!("");

#[program]
pub mod oridion {
    use std::str::FromStr;
    use solana_program::instruction::Instruction;
    use super::*;

    ///-------------------------------------------------------------------///
    /// UNIVERSE SECTION
    ///-------------------------------------------------------------------///
    /// This is the main initialization function.
    /// It will create the universe and will only be called one time.
    ///-------------------------------------------------------------------///
    pub fn bang(ctx: Context<BigBang>) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        let universe: &mut Account<Universe> = &mut ctx.accounts.universe;
        universe.bp = ctx.bumps.universe; 
        universe.p = Vec::<String>::new();
        universe.st = clock.unix_timestamp;
        universe.up = clock.unix_timestamp; 
        universe.cfe = 0; //Comet fee Lamports (0) - Starts at 0 cents (when solana is $100 per 1)
        universe.hpfe = 1000000; //Hop planet lamports (1000000)
        universe.hsfe2 = 2000000; //Hop star lamports (2000000)
        universe.hsfe3 = 3000000; //Hop star lamports (3000000)
        universe.wfe = 0; //Withdraw Lamports (0) - no withdraw fee
        let(pda, _bump_seed) = Pubkey::find_program_address(&[UNIVERSE_PDA_SEED], &ctx.program_id);
        universe.pda = pda;
        msg!("=== BIG BANG! ===");
        Ok(())
    }

    /// -------------------------------------------------------------------///
    /// UPDATE FEE
    ///-------------------------------------------------------------------///
    pub fn update_fee(ctx: Context<UpdateUniverseFee>, comet_fee: u32, hop_planet_fee: u32, hop_star_fee2: u32, hop_star_fee3: u32, withdraw_fee: u32) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        let universe: &mut Account<Universe> = &mut ctx.accounts.universe;
        universe.up = clock.unix_timestamp; //must set this here as well for random comet id
        universe.cfe = comet_fee as u64; //Lamports
        universe.hpfe = hop_planet_fee as u64; //Lamports
        universe.hsfe2 = hop_star_fee2 as u64; //Lamports
        universe.hsfe3 = hop_star_fee3 as u64; //Lamports
        universe.wfe = withdraw_fee as u64; //Lamports
        msg!("=== FEE UPDATED ===");
        Ok(())
    }


    ///-------------------------------------------------------------------///
    /// CREATE PLANET
    ///-------------------------------------------------------------------///
    pub fn create_planet(ctx: Context<CreatePlanet>, name: String) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();
        let planet: &mut Account<Planet> = &mut ctx.accounts.planet;
        let universe: &mut Account<Universe> = &mut ctx.accounts.universe;
        planet.name = name.clone();
        planet.created = clock.unix_timestamp;
        planet.bump = ctx.bumps.planet;
        planet.pda = get_planet_program_address(name.clone(),&ctx.program_id);
        planet.visits = 0;
        //Universe
        universe.p.push(name.clone());
        msg!("=== PLANET {} CREATED!===", name.to_string());
        Ok(())
    }

    ///-------------------------------------------------------------------///
    /// DELETE PLANET
    ///-------------------------------------------------------------------///
    pub fn delete_planet(ctx: Context<DeletePlanet>) -> Result<()> {
        let planet_lamports = ctx.accounts.planet.get_lamports();

        //ERROR! PLANET HAS MONEY STILL IN IT! (1593840 == empty planet) 1593840
        if planet_lamports > 1593840 {
            msg!("Error deleting planet | Lamports: {}", planet_lamports.to_string());
            return Err(errors::ErrorCode::PlanetDeleteHasFundsError.into())
        }

        //Remove planet from universe list
        let universe: &mut Account<Universe> = &mut ctx.accounts.universe;
        let planet: &mut Account<Planet> = &mut ctx.accounts.planet;
        universe.p.retain(|x| x != &planet.name);
        msg!("=== PLANET {} DELETED ===", planet.name.to_string());
        Ok(())
    }



    ///-------------------------------------------------------------------///
    ///-------------------------------------------------------------------///
    /// COMET SECTION
    ///-------------------------------------------------------------------///
    /// -------------------------------------------------------------------///


    ///-------------------------------------------------------------------///
    /// CREATE NEW COMET [USER GENERATED]
    /// Create users COMET to handles initial deposit transaction to galaxy and then hop to initial planet.
    /// - Signed by User
    /// - Deposit (In lamports)
    /// - Transfer to Galaxy and then to Planet
    /// - this comes first before creating deposit account.
    ///-------------------------------------------------------------------///
    pub fn new_comet(ctx: Context<CreateComet>,deposit_lamports: u64) -> Result<()> {

        let planet: &mut Account<Planet> = &mut ctx.accounts.planet;
        let universe: &mut Account<Universe> = &mut ctx.accounts.universe;
        let universe_fee = universe.cfe;

        //Increment visit
        planet.visits += 1;


        //----------------------------------------///
        // TRANSACTION
        //----------------------------------------///

        //SET UP
        let creator_account: &Signer = &ctx.accounts.creator;
        let universe_account: &Account<Universe> = &ctx.accounts.universe;
        let treasury_account: &SystemAccount = &ctx.accounts.treasury;
        let planet_account: &Account<Planet> = &ctx.accounts.planet;


        //If there is a deposit fee, then process fee
        if universe_fee > 0 {
            // FEE TRANSFER
            let fee_transfer_instruction: Instruction = system_instruction::transfer(creator_account.key, treasury_account.to_account_info().key, universe_fee);
            // Invoke the transfer instruction one
            solana_program::program::invoke_signed(
                &fee_transfer_instruction,
                &[
                    creator_account.to_account_info(),
                    treasury_account.to_account_info().clone(),
                    ctx.accounts.system_program.to_account_info(),
                ],
                &[],
            )?;
        }


        // CREATOR DEPOSIT TRANSFER
        // Create the deposit instruction 1
        let transfer_instruction: Instruction = system_instruction::transfer(creator_account.key, universe_account.to_account_info().key, deposit_lamports);
        // Invoke the transfer instruction one
        solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                creator_account.to_account_info(),
                universe_account.to_account_info().clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        msg!("Deposit to galaxy: {} Lamports", deposit_lamports);

        //----------------------------------------///
        // TRANSACTION
        // From universe to planet
        //----------------------------------------///
        //Universe lamports before transfer to planet
        let universe_lamports = &ctx.accounts.universe.get_lamports();
        msg!("Galaxy lamports before hop to planet: {}", universe_lamports);
        msg!("Hopping from galaxy to planet: {}", planet_account.name);
        //msg!("===================================");

        // Do transaction 2 - universe to planet
        ctx.accounts.planet.add_lamports(deposit_lamports)?;
        ctx.accounts.universe.sub_lamports(deposit_lamports)?;

        msg!("Transfer completed");
        msg!("User PDA created and initial hop completed");
        Ok(())
    }


    ///-------------------------------------------------------------------///
    /// HOP COMET TO PLANET
    /// When this function is called, it will generate a star and transfer the funds
    /// either from universe to the star, or from star to star.
    /// it will be connected to
    /// -------------------------------------------------------------------///
    pub fn planet_hop(ctx: Context<PlanetHop>,lamports: u64) -> Result<()>{
        let from: &mut Account<Planet> = &mut ctx.accounts.from_planet;
        let to: &mut Account<Planet> = &mut ctx.accounts.to_planet;

        //-------------------------------------------------
        // VERY IMPORTANT VALIDATIONS.
        //-------------------------------------------------
        // TO AND FROM CANNOT BE THE SAME
        //-------------------------------------------------
        if from.name == to.name {
            //msg!("HOP ERROR: TO AND FROM PLANET CANNOT BE THE SAME!");
            return Err(errors::ErrorCode::HopErrorToAndFromAreSame.into())
        }

        //Increment visits
        to.visits += 1;

        //----------------------------------------///
        // TRANSACTION
        //----------------------------------------///
        ctx.accounts.to_planet.add_lamports(lamports)?;
        ctx.accounts.from_planet.sub_lamports(lamports)?;
        msg!("Secret Planet hop completed");
        Ok(())
    }


    /// WITHDRAW COMET FUNDS TO FINAL DESTINATION.
    /// - THIS ONLY HANDLES THE TRANSACTION FROM PLANET TO FINAL USER WALLET.
    /// - This is just like planet hop except deliver to destination wallet
    pub fn withdraw(ctx: Context<WithdrawAccounts>, withdraw_lamports: u64) -> Result<()> {
        let from: &mut Account<Planet> = &mut ctx.accounts.from_planet;

        //VALIDATION
        let current_from_lamports_balance: u64 = from.get_lamports();
        if current_from_lamports_balance <= withdraw_lamports {
            return Err(errors::ErrorCode::PlanetNotEnoughFundsError.into())
        }

        //----------------------------------------///
        // TRANSACTION - Transfer to destination
        //----------------------------------------///
        ctx.accounts.destination.add_lamports(withdraw_lamports)?;
        ctx.accounts.from_planet.sub_lamports(withdraw_lamports)?;
        msg!("Withdraw success");
        Ok(())
    }


    ///-------------------------------------------------------------------///
    ///-------------------------------------------------------------------///
    /// STAR HOP SECTION
    ///-------------------------------------------------------------------///
    /// -------------------------------------------------------------------///
     pub fn star_hop_two_start(ctx: Context<StarHopTwoStart>, star_one :String , star_two: String, deposit: u64) -> Result<()>{
        let clock: Clock = Clock::get().unwrap();
        let from: &mut Account<Planet> = &mut ctx.accounts.from_planet;
        let manager: &Signer = &ctx.accounts.manager;

        let star1: &mut Account<Star> = &mut ctx.accounts.star_one;
        let star2: &mut Account<Star> = &mut ctx.accounts.star_two;

        let from_planet_name: String = from.name.to_owned();

        //-------------------------------------------------
        // WE NEED TO DO VALIDATIONS
        // STAR ONE AND TWO CANNOT BE THE SAME
        //-------------------------------------------------
        if star_one == star_two {
            //msg!("HOP ERROR: STAR IDS MUST BE UNIQUE!");
            return Err(errors::ErrorCode::HopErrorStarsMustBeUnique.into())
        }

        msg!("Star hop validation successful");
        msg!("Beginning hop");

        // GET DEPOSIT SPLIT AMOUNT
        // First get the percent to split deposit between two stars.
        let clock_time_str = clock.unix_timestamp.to_string();
        let percent = {
            let split_pos = clock_time_str.char_indices().nth_back(1).unwrap().0;
            &clock_time_str[split_pos..]
        };
        msg!("Total deposit lamports: {}", deposit.to_string());
        msg!("Random generated split percentage: %{}", percent.to_string());
        let percent: f32 = f32::from_str(percent).unwrap();

        let star_one_amount: u64 = ((percent / 100f32) * deposit as f32) as u64;
        let star_two_amount: u64 = deposit - star_one_amount;

        msg!("Hopping to star 1: {}", star_one_amount.to_string());
        msg!("Hopping to Star 2: {}", star_two_amount.to_string());

        //Set amounts to accounts
        star1.amount = star_one_amount.clone();
        star2.amount = star_two_amount.clone();
        star1.manager = manager.key().clone();
        star2.manager = manager.key().clone();

        //----------------------------------------///
        // TRANSACTIONS
        //----------------------------------------///
        ctx.accounts.star_one.add_lamports(star_one_amount)?;
        ctx.accounts.star_two.add_lamports(star_two_amount)?;
        ctx.accounts.from_planet.sub_lamports(deposit)?;

        msg!("Hop from planet {} to two stars complete", from_planet_name);
        Ok(())
    }


    ///-------------------------------------------------------------------///
    ///-------------------------------------------------------------------///
    /// STAR HOP SECTION
    ///-------------------------------------------------------------------///
    /// -------------------------------------------------------------------///
     pub fn star_hop_two_end(ctx: Context<StarHopTwoEnd>) -> Result<()>{
        let to: &mut Account<Planet> = &mut ctx.accounts.to_planet;
        let to_planet_name: String = to.name.to_owned();
        let star1: &mut Account<Star> = &mut ctx.accounts.star_one;
        let star2: &mut Account<Star> = &mut ctx.accounts.star_two;
        let star_one_amount: u64 = star1.amount.clone();
        let star_two_amount: u64 = star2.amount.clone();

         //Clear our star amount
        star1.amount = 0;
        star2.amount = 0;

        //Increment planet visit
        to.visits += 1;

        //----------------------------------------///
        // TRANSACTIONS
        // Transaction from stars one and two to destination planet
        //----------------------------------------///
        // msg!("Beginning HOP");
        let total_lamports = star_one_amount + star_two_amount;
        ctx.accounts.star_one.sub_lamports(star_one_amount)?;
        ctx.accounts.star_two.sub_lamports(star_two_amount)?;
        ctx.accounts.to_planet.add_lamports(total_lamports)?;
        msg!("Hop from two stars to planet {} complete", to_planet_name);

        //----------------------------------------///
        // EXPLODE STARS
        //----------------------------------------///
        msg!("Exploding stars..");
        //Transfer out remaining lamports
        let star_one_remaining_lamports = ctx.accounts.star_one.get_lamports();
        let star_two_remaining_lamports = ctx.accounts.star_two.get_lamports();
        ctx.accounts.manager.add_lamports(star_one_remaining_lamports)?;
        ctx.accounts.star_one.sub_lamports(star_one_remaining_lamports)?;
        ctx.accounts.manager.add_lamports(star_two_remaining_lamports)?;
        ctx.accounts.star_two.sub_lamports(star_two_remaining_lamports)?;

        //Clear star 1 data.
        let star_one_account_info = ctx.accounts.star_one.to_account_info();
        let star_two_account_info = ctx.accounts.star_two.to_account_info();

        let mut data_one = star_one_account_info.try_borrow_mut_data()?;
        for byte in data_one.deref_mut().iter_mut() {
            *byte = 0;
        }

        let star_one_dst: &mut [u8] = &mut data_one;
        let mut star_one_cursor = Cursor::new(star_one_dst);
        star_one_cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();

        //Clear star 2 data
        let mut data_two = star_two_account_info.try_borrow_mut_data()?;
        for byte in data_two.deref_mut().iter_mut() {
            *byte = 0;
        }
        let star_two_dst: &mut [u8] = &mut data_one;
        let mut star_two_cursor = Cursor::new(star_two_dst);
        star_two_cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();
        msg!("Stars destroyed");
        // star_one_account_info.assign(&ctx.program_id);
        // star_one_account_info.realloc(0, false)?;
        // star_two_account_info.assign(&ctx.program_id);
        // star_two_account_info.realloc(0, false)?;
        msg!("Star hop completed!");
        Ok(())
    }





    /// STAR HOP THREE
    pub fn star_hop_three_start(ctx: Context<StarHopThreeStart>, star_one :String , star_two: String, star_three: String, deposit: u64) -> Result<()>{
        let clock: Clock = Clock::get().unwrap();
        let from: &mut Account<Planet> = &mut ctx.accounts.from_planet;
        let from_planet_name: String = from.name.to_owned();
        let manager: &Signer = &ctx.accounts.manager;

        let star1: &mut Account<Star> = &mut ctx.accounts.star_one;
        let star2: &mut Account<Star> = &mut ctx.accounts.star_two;
        let star3: &mut Account<Star> = &mut ctx.accounts.star_three;

        //-------------------------------------------------
        // WE NEED TO DO VERY IMPORTANT VALIDATIONS
        // STAR ONE AND TWO CANNOT BE THE SAME
        //-------------------------------------------------
        if star_one == star_two || star_two == star_three || star_one == star_three {
            //msg!("HOP ERROR: STAR IDS MUST BE UNIQUE!");
            return Err(errors::ErrorCode::HopErrorStarsMustBeUnique.into())
        }

        msg!("Star hop validation successful");
        msg!("Beginning hop");


        // GET DEPOSIT SPLIT AMOUNT
        // First get the percent to split deposit between two stars.
        let clock_time_str = clock.unix_timestamp.to_string();
        let first_split_percent = {
            let split_pos = clock_time_str.char_indices().nth_back(1).unwrap().0;
            &clock_time_str[split_pos..]
        };

        msg!("Lamports splitting: {}", deposit.to_string());
        let first_split_percent: f32 = f32::from_str(first_split_percent).unwrap();
        let second_split_percent: f32 = 100f32 - first_split_percent;
        msg!("Random generated split percentage: %{} / %{}", first_split_percent.to_string(), second_split_percent.to_string());

        //Determine side amounts here
        let side_one_amount: u64 = ((first_split_percent / 100f32) * deposit as f32) as u64;
        let side_two_amount: u64 = deposit - side_one_amount;

        let (star_one_amount, star_two_amount, star_three_amount) = if side_one_amount > side_two_amount {
            //Side one is larger so we split side one.
            //Side two is set as star 2
            //We further split side one to create star one and star three
            let one: u64 = ((second_split_percent / 100f32) * side_one_amount as f32) as u64;
            let three: u64 = side_one_amount - one;
            msg!("Third split on first side");
            (one,side_two_amount,three)
        } else {
            //Side two is larger so we split side two.
            //Side one is set as star one
            //We further split side two to create side two and side three
            let two: u64 = ((second_split_percent / 100f32) * side_two_amount as f32) as u64;
            let three: u64 = side_two_amount - two;
            msg!("Third split on second side");
            (side_one_amount,two,three)
        };

        msg!("Hopping to star 1: {}", star_one_amount.to_string());
        msg!("Hopping to star 2: {}", star_two_amount.to_string());
        msg!("Hopping to star 3: {}", star_three_amount.to_string());

        star1.amount = star_one_amount;
        star2.amount = star_two_amount;
        star3.amount = star_three_amount;
        star1.manager = manager.key().clone();
        star2.manager = manager.key().clone();
        star3.manager = manager.key().clone();

        if star_one_amount + star_two_amount + star_three_amount != deposit {
            //msg!("PERCENT CALCULATION ERROR!");
            return Err(errors::ErrorCode::StarHopCalculationError.into())
        }

        // Transfer from planet to star one and two
        ctx.accounts.star_one.add_lamports(star_one_amount)?;
        ctx.accounts.star_two.add_lamports(star_two_amount)?;
        ctx.accounts.star_three.add_lamports(star_three_amount)?;
        ctx.accounts.from_planet.sub_lamports(deposit)?;
        msg!("Hop from planet {} to three stars complete", from_planet_name);
        msg!("Star hop completed");
        Ok(())
    }


    pub fn star_hop_three_end(ctx: Context<StarHopThreeEnd>, deposit: u64) -> Result<()>{
        let to: &mut Account<Planet> = &mut ctx.accounts.to_planet;
        let to_planet_name: String = to.name.to_owned();
        let star1: &mut Account<Star> = &mut ctx.accounts.star_one;
        let star2: &mut Account<Star> = &mut ctx.accounts.star_two;
        let star3: &mut Account<Star> = &mut ctx.accounts.star_three;
        let star_one_amount: u64 = star1.amount.clone();
        let star_two_amount: u64 = star2.amount.clone();
        let star_three_amount: u64 = star3.amount.clone();

         //Clear our star amount
        star1.amount = 0;
        star2.amount = 0;
        star3.amount = 0;

        //Increment planet visit
        to.visits += 1;

        msg!("Beginning hop");
        msg!("Hopping to star 1: {}", star_one_amount.to_string());
        msg!("Hopping to star 2: {}", star_two_amount.to_string());
        msg!("Hopping to star 3: {}", star_three_amount.to_string());

        if star_one_amount + star_two_amount + star_three_amount != deposit {
            //msg!("PERCENT CALCULATION ERROR!");
            return Err(errors::ErrorCode::StarHopCalculationError.into())
        }

        //----------------------------------------///
        // TRANSACTIONS
        //----------------------------------------///

        //Transaction from stars one and two to destination planet
        ctx.accounts.star_one.sub_lamports(star_one_amount)?;
        ctx.accounts.star_two.sub_lamports(star_two_amount)?;
        ctx.accounts.star_three.sub_lamports(star_three_amount)?;
        ctx.accounts.to_planet.add_lamports(deposit)?;
        msg!("Hop from three stars to planet {} complete", to_planet_name);

        //----------------------------------------///
        // EXPLODE STARS
        //----------------------------------------///
        msg!("Exploding stars..");
        //Transfer out remaining lamports
        let star_one_remaining_lamports = ctx.accounts.star_one.get_lamports();
        let star_two_remaining_lamports = ctx.accounts.star_two.get_lamports();
        let star_three_remaining_lamports = ctx.accounts.star_three.get_lamports();
        ctx.accounts.manager.add_lamports(star_one_remaining_lamports)?;
        ctx.accounts.star_one.sub_lamports(star_one_remaining_lamports)?;

        ctx.accounts.manager.add_lamports(star_two_remaining_lamports)?;
        ctx.accounts.star_two.sub_lamports(star_two_remaining_lamports)?;

        ctx.accounts.manager.add_lamports(star_three_remaining_lamports)?;
        ctx.accounts.star_three.sub_lamports(star_three_remaining_lamports)?;

        //Ready clearing data
        let star_one_account_info = ctx.accounts.star_one.to_account_info();
        let star_two_account_info = ctx.accounts.star_two.to_account_info();
        let star_three_account_info = ctx.accounts.star_three.to_account_info();

        //Clear star 1 data.
        let mut data_one = star_one_account_info.try_borrow_mut_data()?;
        for byte in data_one.deref_mut().iter_mut() {
            *byte = 0;
        }

        let star_one_dst: &mut [u8] = &mut data_one;
        let mut star_one_cursor = Cursor::new(star_one_dst);
        star_one_cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();

        //Clear star 2 data
        let mut data_two = star_two_account_info.try_borrow_mut_data()?;
        for byte in data_two.deref_mut().iter_mut() {
            *byte = 0;
        }
        let star_two_dst: &mut [u8] = &mut data_two;
        let mut star_two_cursor = Cursor::new(star_two_dst);
        star_two_cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();


        //Clear star 3 data
        let mut data_three = star_three_account_info.try_borrow_mut_data()?;
        for byte in data_three.deref_mut().iter_mut() {
            *byte = 0;
        }
        let star_three_dst: &mut [u8] = &mut data_three;
        let mut star_three_cursor = Cursor::new(star_three_dst);
        star_three_cursor.write_all(&CLOSED_ACCOUNT_DISCRIMINATOR).unwrap();
        msg!("Stars destroyed");
        // star_one_account_info.assign(&ctx.program_id);
        // star_one_account_info.realloc(0, false)?;
        // star_two_account_info.assign(&ctx.program_id);
        // star_two_account_info.realloc(0, false)?;
        msg!("Star hop completed");
        Ok(())
    }

}
