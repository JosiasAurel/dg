use std::collections::HashMap;

type Response = Vec<HashMap<String, Vec<HashMap<String, Vec<HashMap<String, String>>>>>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args: Vec<String> = std::env::args().skip(1).collect();
    let word = args.get(0).expect("Please enter a word");

    let url = format!("https://api.dictionaryapi.dev/api/v2/entries/en/{}", word);

    let response = reqwest::blocking::get(&url)?;
    let result = response.json::<Response>()?;

    println!("{:#?}", result);

    Ok(())
}
