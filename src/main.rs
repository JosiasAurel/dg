// use std::collections::HashMap;
use serde_json::Value;

// type Response = Vec<HashMap<String, Vec<HashMap<String, Vec<HashMap<String, String>>>>>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word = args.get(0).expect("Please enter a word");

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response = reqwest::blocking::get(&url)?;
    let result = response.json::<Value>()?;

    let phonetic = &result[0]["phonetic"];
    let part_of_speech = &result[0]["meanings"][0]["partOfSpeech"];
    let defs = &result[0]["meanings"][0]["definitions"];

    println!("Phonetic: {}", phonetic);
    println!("Part of speech: {}", part_of_speech);
    println!("");
    println!("Definitions");
    println!("");

    let mut count = 0;

    loop {
        if count == 3 {
            break;
        }
      println!("Definition -> {}", defs[count]["definition"]);
      count += 1;
    }

    Ok(())
}
