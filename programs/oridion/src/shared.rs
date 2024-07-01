use super::*;
pub fn get_planet_program_address(planet_name: &String, program_id: &Pubkey) -> Pubkey {
    let(pk, _pda_bump) = Pubkey::find_program_address(&[
        PLANET_PDA_SEED_PRE,
        planet_name.as_ref(),
        PLANET_PDA_SEED_POST
    ], program_id);
    pk
}

pub fn get_random_percent() -> f32 {
    let clock: Clock = Clock::get().unwrap();
    // First get the percent to split deposit between two stars.
    let clock_time_str: String = clock.unix_timestamp.to_string();
    let percent: &str = {
        let split_pos: usize = clock_time_str.char_indices().nth_back(1).unwrap().0;
        &clock_time_str[split_pos..]
    };

    //msg!("Lamports splitting: {}", deposit.to_string());
    //let first_split_percent: f32 = f32::from_str(first_split_percent).unwrap();
    let percent: f32 = percent.parse::<f32>().unwrap();
    if percent < 10 as f32 {
        return 10 as f32
    } else if percent > 90 as f32 {
        return 90 as f32
    } else {
        return percent
    }
}