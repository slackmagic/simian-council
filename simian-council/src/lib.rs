pub mod pokemon_generator;
pub mod secret_generator;
pub mod utils;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub fn ask_for_a_secret(num: u8, start_enthro_level: u8, end_enthro_level: u8) -> String {
    secret_generator::generate(num, start_enthro_level, end_enthro_level)
}

pub fn ask_for_a_pokemon() -> String {
    pokemon_generator::generate_with_adjective()
}
