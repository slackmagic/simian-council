use crate::utils::file_interactor::*;
use rand::prelude::*;
use std::error::Error;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let path = Path::new("./resources/pokemon/fr.json");

    let pokemon_data = get_file_as_string(path);
    let list: Vec<String> = serde_json::from_str(&pokemon_data).unwrap();

    let random_index = rng.gen_range(1, list.len());
    list[random_index].to_string()
}

pub fn generate_with_adjective() -> String {
    let mut rng = rand::thread_rng();
    let result: String = generate();
    let mut dictionary: Vec<String> = Vec::new();
    let path = Path::new("./resources/word/en_adjectives_list.txt");

    for line in BufReader::new(get_file(&path)).lines() {
        match line {
            Err(why) => panic!("couldn't read line: {}", why.description()),
            Ok(line) => dictionary.push(line),
        }
    }

    let random_index = rng.gen_range(1, dictionary.len());
    dictionary[random_index].to_string() + " " + &result
}
