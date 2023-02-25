use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct WordInfo {
    phonetic: String,
    part_of_speech: String,
    definitions: Vec<String>,
}

type Dictionary = HashMap<String, WordInfo>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dictionary_name: &str = "/.dg-dict.json";
    let dir = env::current_exe()?;
    let dictionary_path = &format!(
        "{}{}",
        dir.to_str().unwrap().trim_end_matches("dg"),
        dictionary_name
    );
    println!("Path = {}", dictionary_path);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word = args.get(0).expect("Please enter a word");

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    dg_routine(dictionary_path, word, &url);

    Ok(())
}

fn dg_routine(dictionary_path: &str, word: &String, url: &str) {
    let file_contents = fs::read_to_string(dictionary_path).unwrap_or_else(|_| String::from(""));

    let mut dictionary: Dictionary =
        serde_json::from_str(&file_contents).unwrap_or_else(|_| Dictionary::new());

    if dictionary.contains_key(word) {
        let word_info = dictionary
            .get(word)
            .expect("Failed to get word definition from local database");
        print_word_info(word_info);
    } else {
        let response = reqwest::blocking::get(url).expect("Failed to fetch");
        let result = response
            .json::<Value>()
            .expect("Failed to parse JSON response");

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
            definitions: deserialized_defs,
        };

        dictionary.insert(String::from(word), word_and_def.clone());
        let dictionary_str = serde_json::to_string(&dictionary).expect("Failed to save updates");

        fs::write(dictionary_path, dictionary_str).expect("Failed to write to dictionary");

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
