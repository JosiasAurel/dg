use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use serde::{Serialize, Deserialize};

// type Response = Vec<HashMap<String, Vec<HashMap<String, Vec<HashMap<String, String>>>>>>;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
struct WordInfo {
    phonetic: String,
    part_of_speech: String,
    definitions: Vec<String>
}

type Dictionary = HashMap<String, WordInfo>;

const DICTIONARY_PATH: &str = "./dictionary.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word = args.get(0).expect("Please enter a word");

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);
    
    dg_routine(word, &url);
    
    
    Ok(())
}

fn dg_routine(word: &String, url: &str) {

    let file_contents = fs::read_to_string(DICTIONARY_PATH).unwrap_or_else(|_| String::from(""));

    let mut dictionary: Dictionary = serde_json::from_str(&file_contents).unwrap_or_else(|_| Dictionary::new());

    if dictionary.contains_key(word) {
        let word_info = dictionary.get(word).expect("Failed to get word definition from local database");
        print_word_info(word_info);
    } else {
        let response = reqwest::blocking::get(url).expect("Failed to fetch");
        let result = response.json::<Value>().expect("Failed to parse JSON response");

        let phonetic = &result[0]["phonetic"];
        let part_of_speech = &result[0]["meanings"][0]["partOfSpeech"];
        let defs = &result[0]["meanings"][0]["definitions"];

        let mut deserialized_defs = Vec::new();


        let mut count = 0;
        loop {
            if count == 3 {
                break;
            }
            deserialized_defs.push(defs[count]["definition"].to_string());
            count += 1;
        }

        let word_and_def: WordInfo = WordInfo {
            phonetic: phonetic.to_string(),
            part_of_speech: part_of_speech.to_string(),
            definitions: deserialized_defs
        };

        dictionary.insert(String::from(word), word_and_def.clone());
        let dictionary_str = serde_json::to_string(&dictionary).expect("Failed to save updates");
                
        fs::write(DICTIONARY_PATH, dictionary_str).expect("Failed to write to dictionary");
            
        print_word_info(&word_and_def);
    }
}

fn print_word_info(word_info: &WordInfo) {
    println!("Phonetic: {}", word_info.phonetic);
    println!("Part of speech: {}", word_info.part_of_speech);
    println!();
    println!("Definitions");
    println!();


    for definition in word_info.definitions.iter() {
        println!("Definition -> {}", definition);
    }
}
