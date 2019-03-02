pub mod dictionary_entry;
use crate::secret_generator::dictionary_entry::*;
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn generate(num: u8, start_enthro_level: u8, end_enthro_level: u8) -> String {
    let dictionary = get_dictionary();
    let mut rng = rand::thread_rng();
    let mut secret: String = String::new();
    let range = get_range(end_enthro_level, dictionary.len());
    println!("range selected {:?}", range);

    let (filtered_dictionary, _) = dictionary.split_at(range);

    for _ in 0..num {
        let random_index = rng.gen_range(1, filtered_dictionary.len());
        secret += &filtered_dictionary[random_index].word;
        secret += " ";
    }

    secret
}

fn get_range(mut enthropy_level: u8, max_size: usize) -> usize {
    if enthropy_level > 100 {
        enthropy_level = 100;
    }
    let percent: f32 = (enthropy_level as f32) / 100.0;
    (max_size as f32 * percent) as usize
}

fn get_dictionary() -> Box<Vec<DictionaryEntry>> {
    let mut dictionary: Vec<DictionaryEntry> = Vec::new();
    let path = Path::new("./resources/wordlist");

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    for line in BufReader::new(file).lines() {
        match line {
            Err(why) => panic!("couldn't read line: {}", why.description()),
            Ok(line) => {
                let entry: Vec<&str> = line.split(' ').collect();

                dictionary.push(DictionaryEntry::new(
                    entry[0].to_string(),
                    entry[1].to_string(),
                ));
            }
        }
    }

    Box::new(dictionary)
}

fn get_50k_dictionary() -> Box<Vec<DictionaryEntry>> {
    let mut dictionary: Vec<DictionaryEntry> = Vec::new();
    let path = Path::new("./resources/en_50k.txt");

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    for line in BufReader::new(file).lines() {
        match line {
            Err(why) => panic!("couldn't read line: {}", why.description()),
            Ok(line) => {
                let entry: Vec<&str> = line.split(' ').collect();

                dictionary.push(DictionaryEntry::new(
                    entry[1].to_string(),
                    entry[0].to_string(),
                ));
            }
        }
    }

    Box::new(dictionary)
}
