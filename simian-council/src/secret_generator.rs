pub mod dictionary_entry;

use crate::secret_generator::dictionary_entry::*;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_dictionary() -> Box<Vec<DictionaryEntry>> {
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

pub fn get_50k_dictionary() -> Box<Vec<DictionaryEntry>> {
    let mut dictionary: Vec<DictionaryEntry> = Vec::new();
    let path = Path::new("./resources/fr_50k.txt");

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
