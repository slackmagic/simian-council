use crate::secret_generator::*;
use rand::prelude::*;

pub mod secret_generator;

pub fn ask_for_a_secret(num: u8, bits: u8) -> String {
    let dictionary = get_50k_dictionary();
    let mut rng = rand::thread_rng();
    let mut secret: String = String::new();

    let index = (2 as usize).pow(bits as u32);
    let (filtered_dictionary, _) = dictionary.split_at(index);

    for _ in 0..num {
        let random_index = rng.gen_range(1, filtered_dictionary.len());
        secret += &filtered_dictionary[random_index].word;
        secret += " ";
    }

    secret
}
