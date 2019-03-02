pub mod secret_generator;

pub fn ask_for_a_secret(num: u8, start_enthro_level: u8, end_enthro_level: u8) -> String {
    secret_generator::generate(num, start_enthro_level, end_enthro_level)
}
