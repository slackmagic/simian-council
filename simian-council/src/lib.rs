use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn ask_for_a_secret(num: u8, bits: u8) -> String {
    let dictionary = get_dictionary();
    let (filtered_dictionary, _) = dictionary.split_at((2 as usize).pow(bits as u32));

    let mut rng = rand::thread_rng();
    let mut secret: String = "".to_string();

    for _ in 0..num {
        let random_index = rng.gen_range(1, filtered_dictionary.len());
        let entry: Vec<&str> = filtered_dictionary[random_index].split(' ').collect();
        secret += entry[1];
        secret += " ";
    }

    secret
}

fn get_dictionary() -> Vec<String> {
    let mut dictionary: Vec<String> = Vec::new();
    let path = Path::new("./resources/wordlist");

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    for line in BufReader::new(file).lines() {
        match line {
            Err(why) => panic!("couldn't read line: {}", why.description()),
            Ok(line) => dictionary.push(line),
        }
    }

    dictionary
}
