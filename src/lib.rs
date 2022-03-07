//! # wordle_solver
//!
//! Wordle solver is a small library as well as an example utility (`wordle-solver`)
//! that helps you win the daily wordle ( <https://www.powerlanguage.co.uk/wordle/> )
//!
//! See the solver module and the Solver struct
//!

pub use solver::*;

pub mod solver;

use assets_manager::{
    loader::{LoadFrom, StringLoader},
    Asset, AssetCache,
};

use regex::Regex;
use std::collections::HashSet;
use std::error::Error;

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

pub fn load_words(cache: &AssetCache, fname: &str, size: u8) -> Result<HashSet<String>, Box<dyn Error>> {
    let bd = cache.load::<AssetString>(fname)?.read();
    let s = &bd.0;

    let regex_string = format!("^[[:alpha:]]{{{}}}$", size);
    let nchars: Regex = Regex::new(&regex_string).unwrap();

    // The only interesting words are 5 characters, and all letters.
    let words: HashSet<String> = s
        .lines()
        .map(|rs| rs.to_uppercase()) // turn all refs into a string.
        .filter(|rs| nchars.is_match(rs))
        .collect();
    Ok(words)
}
