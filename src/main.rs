use assets_manager::AssetCache;
use std::error::Error;
use std::io;
use std::str::FromStr;
use wordle_solver::solver::{FullHint, Hint, Solver};

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
    println!("Building resolver");
    let mut solver = Solver::new(&words);

    let stdin = io::stdin(); // We get `Stdin` here.

    while solver.n_candidates() >= 1 {
        let candidate = solver.first_candidate().unwrap().clone();
        println!(
            "Try this: {:?} ({} possibilities left)",
            candidate,
            solver.n_candidates()
        );
        println!(
            "Feedback? (x=Invalid, e=Exists, g=Good, [enter]=Invalid word):\n{}",
            candidate
        );
        let mut input: String = String::new();
        stdin.read_line(&mut input)?;
        let feedback_str = input.trim();
        if feedback_str.is_empty() {
            // println!("Discarding '{}'", candidate);
            solver.discard_word(&candidate);
        } else {
            // feedback string IS something. Feed the feedback in the solver.
            let full_hints: Vec<FullHint> = feedback_str
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let hint = Hint::from_str(&c.to_string())?;
                    // And add the hint to the solver.
                    let candidate_char =
                        candidate.chars().nth(i).ok_or("To many feedback letters")?;
                    // Note: Invalid hint can only be about letters that are NOT exists or well placed in the word.
                    Ok(FullHint { c: candidate_char, p: i, hint })
                    //solver.add_hint(&candidate_char,&i,hint);
                })
                .map(|r: Result<FullHint, Box<dyn Error>>| r.unwrap())
                .collect();

            solver.ingest_hints(full_hints);
        }
    }
    println!("No more candidates left. Did you win?");

    Ok(())
}
