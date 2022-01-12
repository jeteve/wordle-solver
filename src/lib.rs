use assets_manager::{
    loader::{LoadFrom, StringLoader},
    Asset, AssetCache,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

lazy_static! {
    static ref FIVECHARS: Regex = Regex::new("[[:alpha:]]{5}").unwrap();
}

// The wrapper around the whole string dictionary, with a From trait
// to be able to load it from an asset
struct AssetString(String);
impl From<String> for AssetString {
    fn from(s: String) -> AssetString {
        AssetString(s)
    }
}
impl Asset for AssetString {
    const EXTENSION: &'static str = "txt";
    type Loader = LoadFrom<String, StringLoader>;
}

pub fn load_words(cache: &AssetCache, fname: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let bd = cache.load::<AssetString>(fname)?.read();
    let s = &bd.0;

    // The only interesting words are 5 characters, and all letters.
    let words: HashSet<String> = s
        .lines()
        .map(|rs| rs.to_uppercase()) // turn all refs into a string.
        .filter(|rs| FIVECHARS.is_match(rs))
        .collect();
    Ok(words)
}
