use solana_program::{pubkey, pubkey::Pubkey};

pub const TREASURY_PUBKEY: Pubkey = pubkey!("ordkZKeCDeo8u4Ep5WbiWoXhAU9pzxQZm7FHpKskiTx");

pub const MANAGER_PUBKEY: Pubkey = pubkey!("ordnd8TZFYW4k4MeLrR3qSwXMxezL6W3WryUPYTzLQM");
pub const UNIVERSE_PDA_SEED: &[u8] = b"_x_ORIDION_x_";

//PLANET
pub const PLANET_PDA_SEED_PRE: &[u8] = b"_PLA_";
pub const PLANET_PDA_SEED_POST: &[u8] = b"_NET_";

//Constants for Star seed
pub const STAR_SEED_PRE: &[u8] = b"_ST_";
pub const STAR_SEED_POST: &[u8] = b"_AR_";

/// Constants for sizing properties.
pub  const DISCRIMINATOR_LENGTH: usize = 8;
pub const PUBLIC_KEY_LENGTH: usize = 32;
pub const STAR_ID_LENGTH: usize = 8 * 4; // 8 chars max.
pub const PLANET_NAME: usize = 10 * 4; //10 characters long
pub const TIMESTAMP_LENGTH: usize = 8;
pub const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
pub const LAMPORT_LENGTH: usize = 8; //u64 = 8 bytes
pub const U8_LENGTH: usize = 1; //BUMP = u8 1 = byte
pub const U64_LENGTH: usize = 8; //8 = bytes

//Planet count length
pub const PLANETS_VEC_LENGTH: usize = 104; // 4 + ((4 + 6) * 10)
