use std::error::Error;
use assets_manager::AssetCache;

fn main() -> Result<(), Box<dyn Error>>{
    // load all words from the dictionary, only the ones that are 5 letters
    // build indices, by letter and by (letter,position)

    let fname = "american-english";
    println!("Loading {} dictionary", fname);
    let assets_cache = AssetCache::new("assets")?;
    let words = wordle_solver::load_words(&assets_cache, fname)?;
    println!("Loaded {} words", words.len());

    // main program loop
    // output best guess, get feedback, repeat

    Ok(())
}
