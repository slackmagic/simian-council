pub mod secret_generator;

pub fn ask_for_a_secret(num: u8, bits: u8) -> String {
    secret_generator::generate(num, bits)
}
