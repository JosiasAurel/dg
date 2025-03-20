use miniserde::{json::Value, Deserialize, Serialize};
use std::collections::HashMap;
use std::{env, fs, process};

#[derive(Serialize, Deserialize, Clone)]
struct WordInfo {
    phonetic: String,
    part_of_speech: String,
    definitions: Vec<String>,
}

impl std::fmt::Display for WordInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Phonetic: {}", self.phonetic)?;
        writeln!(f, "Part of speech: {}", self.part_of_speech)?;
        writeln!(f)?;
        writeln!(f, "Definitions")?;
        writeln!(f)?;

        for definition in &self.definitions {
            writeln!(f, "Definition -> {definition:?}")?;
        }
        Ok(())
    }
}

type Dictionary = HashMap<String, WordInfo>;
type Res<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Res<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (dictionary_path, word) = parse_opts(&args)?;

    let word_info = get_word_info(&dictionary_path, word)?;
    println!("{word_info}");
    Ok(())
}

fn parse_opts(args: &[String]) -> Res<(String, &String)> {
    let dictionary_path = if let Ok(env_dict_path) = env::var("DG_DICT_PATH") {
        env_dict_path
    } else {
        const DICTIONARY_NAME: &str = ".dg-dict.json";
        format!("{}/{DICTIONARY_NAME}", env::current_dir()?.display())
    };
    let Some(word) = args.first() else {
        eprintln!("missing word to define");
        process::exit(0x0100);
    };
    Ok((dictionary_path, word))
}

fn get_word_info(dictionary_path: &str, word: &str) -> Res<WordInfo> {
    let cache_content = fs::read_to_string(dictionary_path).unwrap_or_default();

    let mut dictionary: Dictionary = miniserde::json::from_str(&cache_content).unwrap_or_default();
    if let Some(cache_hit) = dictionary.get(word) {
        return Ok(cache_hit.clone());
    }

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}");
    let response = minreq::get(url).with_timeout(8).send()?;
    if response.status_code == 404 {
        return Err(format!("no known definition found for '{word}', check typo").into());
    }
    let json = response.as_str()?;
    let result: Vec<Value> = miniserde::json::from_str(json)?;
    let first_dict_item = result.first().ok_or("no definitions received")?;
    let Value::Object(first_dict_item) = first_dict_item else {
        return Err("got unknown dictionary item".into());
    };
    let fallback = String::from("null");
    let phonetic = if let Some(Value::String(phon)) = first_dict_item.get("phonetic") {
        phon
    } else {
        &fallback
    };
    let Some(Value::Array(meanings)) = first_dict_item.get("meanings") else {
        return Err("received unknown kind of meanings".into());
    };
    let Some(Value::Object(first_meaning)) = meanings.first() else {
        return Err("received unknown kind of definition".into());
    };

    let part_of_speech = if let Some(Value::String(pos)) = first_meaning.get("partOfSpeech") {
        pos
    } else {
        &fallback
    };
    let Some(Value::Array(defs)) = first_meaning.get("definitions") else {
        return Err("no known definitions found".into());
    };
    let deserialized_defs = defs.iter().take(3).map(|raw_definition| {
        let mut definition = &fallback;
        if let Value::Object(def) = raw_definition {
            if let Some(Value::String(def)) = def.get("definition") {
                definition = def;
            }
        }
        definition
    });

    let word_and_def = WordInfo {
        phonetic: phonetic.clone(),
        part_of_speech: part_of_speech.clone(),
        definitions: deserialized_defs.cloned().collect(),
    };

    dictionary.insert(String::from(word), word_and_def.clone());
    let dictionary_str = miniserde::json::to_string(&dictionary);
    fs::write(dictionary_path, dictionary_str)?;

    Ok(word_and_def)
}
