use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref EMPTYSET: HashSet<String> = HashSet::new();
}
pub struct Solver {
    candidates: HashSet<String>,
    well_placed: Vec<(char, usize)>,
    exists: Vec<(char, usize)>,
    invalid: Vec<char>,
    by_letter: HashMap<char, HashSet<String>>,
    by_letter_position: HashMap<(char, usize), HashSet<String>>,
}

pub enum Hint {
    WellPlaced,
    Exists,
    Invalid,
}

impl Solver {
    pub fn new(all_words: &HashSet<String>) -> Solver {
        let candidates = all_words.clone();
        let by_letter = candidates.iter().fold(HashMap::new(), |mut h, v| {
            for c in v.chars() {
                let entry = h.entry(c).or_insert(HashSet::new());
                entry.insert(v.clone());
            }
            h
        });
        let by_letter_position = candidates.iter().fold(HashMap::new(), |mut h, v| {
            for (p, c) in v.chars().enumerate() {
                let entry = h.entry((c, p)).or_insert(HashSet::new());
                entry.insert(v.clone());
            }
            h
        });
        let well_placed = Vec::new();
        let exists = Vec::new();
        let invalid = Vec::new();
        Solver {
            candidates,
            well_placed,
            exists,
            invalid,
            by_letter,
            by_letter_position,
        }
    }

    pub fn n_candidates(&self) -> usize {
        self.candidates.len()
    }

    pub fn first_candidate(&self) -> Option<&String> {
        return self.candidates.iter().next();
    }
    pub fn with_letter(&self, l: &char) -> &HashSet<String> {
        self.by_letter.get(l).unwrap_or(&EMPTYSET)
    }
    pub fn with_letter_in_position(&self, l: &char, p: &usize) -> &HashSet<String> {
        self.by_letter_position.get(&(*l, *p)).unwrap_or(&EMPTYSET)
    }
    pub fn add_hint(&mut self, l: &char, p: &usize, h: Hint) {
        match h {
            Hint::WellPlaced => self.add_well_placed(l, p),
            Hint::Exists => self.add_exists(l, p),
            Hint::Invalid => self.add_invalid(l),
        }
    }
    pub fn discard_word(&mut self,s: &str){
        self.candidates.remove(s);
    }

    fn add_well_placed(&mut self, l: &char, p: &usize) {
        self.well_placed.push((*l, *p));
        // Well place means we need to include only in candidate
        // those with the letter in the right place.
        self.candidates = self
            .candidates
            .intersection(self.with_letter_in_position(l, p))
            .map(|s| s.clone())
            .collect()
    }

    fn add_exists(&mut self, l: &char, p: &usize) {
        self.exists.push((*l, *p));
        // Intersect with the word who just have the character
        let mut new_candidates: HashSet<String> = self
            .candidates
            .intersection(self.with_letter(l))
            .map(|s| s.clone())
            .collect();
        // And REMOVE the words that have this letter in this position
        // if it was well placed, add_well_placed would be called instead.
        let to_remove = self.with_letter_in_position(l, p);
        new_candidates.retain(|s| !to_remove.contains(s));

        self.candidates = new_candidates;
    }

    fn add_invalid(&mut self, l: &char) {
        self.invalid.push(*l);
        // Character is invalid. simply remove all the words containing it
        let to_remove = self.with_letter(l).clone();
        self.candidates.retain(|s| !to_remove.contains(s));
    }

    // kept for reference. Remove when it all works.
    #[deprecated]
    pub fn refresh_candidates(&mut self) {
        // intersect of all the well placed ones from by_letter_position
        // and all the exists ones from by_letter
        // remove the words from invalid ones.
        let well_placed: Vec<&HashSet<String>> = self
            .well_placed
            .iter()
            .map(|(l, p)| self.with_letter_in_position(l, p))
            .collect();
        let exists: Vec<&HashSet<String>> = self
            .exists
            .iter()
            .map(|(l, _)| self.with_letter(l))
            .collect();
        let only_exists_pos: Vec<&HashSet<String>> = self
            .exists
            .iter()
            .map(|(l, p)| self.with_letter_in_position(l, p))
            .collect();
        let invalid: Vec<&HashSet<String>> =
            self.invalid.iter().map(|l| self.with_letter(l)).collect();

        //println!("Exist: {:?}", exists);

        // Start with the exist letter ones as intersection with the original candidates.
        let mut new_candidates: HashSet<String> =
            exists.iter().fold(self.candidates.clone(), |a, &e| {
                a.intersection(e).map(|s| s.clone()).collect()
            });

        // TODO: Existing ONLY in one position means we need to exclude the words that have
        // this letter in the given position.
        new_candidates = only_exists_pos.iter().fold(new_candidates, |mut a, &e| {
            for invalid in e {
                a.remove(invalid);
            }
            a
        });

        // then it has to intersect with all the well placed ones too.
        // println!("Well Placed: {:?}", well_placed);
        new_candidates = well_placed.iter().fold(new_candidates, |a, &e| {
            a.intersection(e).map(|s| s.clone()).collect()
        });

        // Exclude the invalid sets from the new_candidatees
        new_candidates = invalid.iter().fold(new_candidates, |mut a, &e| {
            for invalid in e {
                a.remove(invalid);
            }
            a
        });

        // println!("New candidates: {:?}", new_candidates);

        self.candidates = new_candidates;
    }
}
