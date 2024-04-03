use super::*;
pub fn get_planet_program_address(planet_name: String, program_id: &Pubkey) -> Pubkey {
    let(pk, _pda_bump) = Pubkey::find_program_address(&[
        PLANET_PDA_SEED_PRE,
        planet_name.as_ref(),
        PLANET_PDA_SEED_POST
    ], program_id);
    pk
}
