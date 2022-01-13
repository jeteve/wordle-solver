use assets_manager::AssetCache;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // load all words from the dictionary, only the ones that are 5 letters
    // build indices, by letter and by (letter,position)

    let fname = "american-english";
    println!("Loading {} dictionary", fname);
    let assets_cache = AssetCache::new("assets")?;
    let words = wordle_solver::load_words(&assets_cache, fname)?;
    println!("Loaded {} 5 letter words", words.len());

    // main program loop
    // output best guess, get feedback, repeat

    Ok(())
}
