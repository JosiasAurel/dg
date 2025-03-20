use miniserde::{json::Value, Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, process};

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
    // println!("Path = {}", dictionary_path);
    let args: Vec<String> = std::env::args().skip(1).collect();
    let fallback = String::from("[DEFAULT]");
    let word = args.get(0).unwrap_or(&fallback);

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    dg_routine(dictionary_path, word, &url);

    Ok(())
}

fn dg_routine(dictionary_path: &str, word: &String, url: &str) {
    if word == &String::from("[DEFAULT]") {
        println!("Missing word to define");
        process::exit(0x0100);
    }
    let file_contents = fs::read_to_string(dictionary_path).unwrap_or_else(|_| String::from(""));

    let mut dictionary: Dictionary =
        miniserde::json::from_str(&file_contents).unwrap_or_else(|_| Dictionary::new());

    if dictionary.contains_key(word) {
        let word_info = dictionary
            .get(word)
            .expect("Failed to get word definition from local database");
        print_word_info(word_info);
    } else {
        let response = minreq::get(url).send().expect("Failed to fetch");
        let json = response
            .as_str()
            .expect("Failed to convert response to text");
        eprintln!("response as json: {json:?}");
        let res: Vec<Value> =
            miniserde::json::from_str(json).expect("Failed to parse JSON response");
        eprintln!("response as parsed json: {res:#?}");
        let first_dict_item = res.first().expect("got unknown object");
        let Value::Object(first_dict_item) = first_dict_item else {
            panic!("got unknown object");
        };
        let fallback = String::from("null");
        let phonetic = if let Some(Value::String(phon)) = first_dict_item.get("phonetic") {
            phon
        } else {
            &fallback
        };
        let Some(Value::Array(meanings)) = first_dict_item.get("meanings") else {
            panic!("unknown meanings returned");
        };
        let Some(Value::Object(first_meaning)) = meanings.first() else {
            panic!("unknown definition got");
        };

        let part_of_speech = if let Some(Value::String(pos)) = first_meaning.get("partOfSpeech") {
            pos
        } else {
            &fallback
        };
        let Some(Value::Array(defs)) = first_meaning.get("definitions") else {
            panic!("no known definitions found");
        };
        let deserialized_defs = defs.iter().take(3).map(|d| {
            if let Value::Object(def) = d {
                if let Some(Value::String(def)) = def.get("definition") {
                    def.clone()
                } else {
                    fallback.clone()
                }
            } else {
                fallback.clone()
            }
        });

        let word_and_def: WordInfo = WordInfo {
            phonetic: phonetic.to_string(),
            part_of_speech: part_of_speech.to_string(),
            definitions: deserialized_defs.collect(),
        };

        dictionary.insert(String::from(word), word_and_def.clone());
        let dictionary_str = miniserde::json::to_string(&dictionary);

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
