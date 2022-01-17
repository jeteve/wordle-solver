use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;

lazy_static! {
    static ref EMPTYSET: HashSet<&'static String> = HashSet::new();
}

/// A wordle solver build from a dictionary
pub struct Solver<'a> {
    candidates: RefCell<HashSet<&'a String>>,
    exists_letters: RefCell<HashSet<char>>,
    by_letter: HashMap<char, HashSet<&'a String>>,
    by_letter_position: HashMap<(char, usize), HashSet<&'a String>>,
}

/// A convenience struct to give a hint about
/// a letter in the given position
#[derive(Clone)]
pub struct Hint {
    pub hint: HintType,
    pub letter: char,
    pub position: usize,
}

/// The nature of the hint given by wordle
#[derive(Clone, PartialEq)]
pub enum HintType {
    WellPlaced,
    Exists,
    Invalid,
}

#[derive(Debug)]
pub enum HintParseError {
    InvalidCode(String),
}
impl fmt::Display for HintParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse hint '{:?}'", self)
    }
}
impl error::Error for HintParseError {}

impl std::str::FromStr for HintType {
    type Err = HintParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(HintType::Invalid),
            "e" => Ok(HintType::Exists),
            "g" => Ok(HintType::WellPlaced),
            _ => Err(HintParseError::InvalidCode(s.to_string())),
        }
    }
}

impl<'a> Solver<'a> {
    /// Builds a new Solver
    /// # Example
    /// ```
    ///  use std::collections::HashSet;
    ///  use wordle_solver::solver::Solver;
    ///
    ///  let dictionary = ["class", "clock"].iter().map(|&s| String::from(s))
    ///                   .collect::<HashSet<String>>();
    ///  let solver = Solver::new(&dictionary);
    /// ```
    pub fn new(dictionary: &HashSet<String>) -> Solver {
        let candidates = dictionary.iter().collect::<HashSet<&String>>();
        let by_letter = dictionary.iter().fold(HashMap::new(), |mut h, v| {
            for c in v.chars() {
                let entry = h.entry(c).or_insert_with(HashSet::new);
                entry.insert(v);
            }
            h
        });
        let by_letter_position = dictionary.iter().fold(HashMap::new(), |mut h, v| {
            for (p, c) in v.chars().enumerate() {
                let entry = h.entry((c, p)).or_insert_with(HashSet::new);
                entry.insert(v);
            }
            h
        });
        let exists_letters = HashSet::new();
        Solver {
            candidates: RefCell::new(candidates),
            exists_letters: RefCell::new(exists_letters),
            by_letter,
            by_letter_position,
        }
    }

    /// The current size of the words candidates pool
    pub fn n_candidates(&self) -> usize {
        self.candidates.borrow().len()
    }

    /// The first candidate. Try this in wordle
    pub fn first_candidate(&self) -> Option<&String> {
        return self.candidates.borrow().iter().next().copied();
    }

    fn with_letter(&self, l: &char) -> &HashSet<&String> {
        self.by_letter.get(l).unwrap_or(&EMPTYSET)
    }
    fn with_letter_in_position(&self, l: &char, p: &usize) -> &HashSet<&String> {
        self.by_letter_position.get(&(*l, *p)).unwrap_or(&EMPTYSET)
    }

    /// Ingests a bunch of Hints together,
    /// ensuring logical consistency between them.
    pub fn ingest_hints(&mut self, fhs: Vec<Hint>) {
        let (valid, invalid): (Vec<_>, Vec<_>) = fhs.iter().partition(|&h| h.hint != HintType::Invalid);
        for fh in valid {
            self.add_hint(fh.clone());
        }
        for fh in invalid {
            self.add_hint(fh.clone());
        }
    }

    /// Just add one Hint
    pub fn add_hint(&mut self, fh: Hint) {
        match fh.hint {
            HintType::WellPlaced => self.add_well_placed(&fh.letter, &fh.position),
            HintType::Exists => self.add_exists(&fh.letter, &fh.position),
            HintType::Invalid => self.add_invalid(&fh.letter),
        }
    }

    /// In case you dont want to use the Hint struct
    pub fn add_raw_hint(&mut self, l: &char, p: &usize, h: HintType) {
       self.add_hint(Hint{hint: h, letter: *l, position: *p})
    }

    /// Some words might be in your dictionary but not
    /// in wordle. Use this to discard them
    pub fn discard_word(&self, s: &str) {
        self.candidates.borrow_mut().remove(&s.to_string());
    }

    fn add_well_placed(&self, l: &char, p: &usize) {
        //println!("Restricting to words containing an {} at position {}" , l, p);
        let to_retain = self.with_letter_in_position(l, p).clone();
        self.candidates
            .borrow_mut()
            .retain(|s| to_retain.contains(s));
        self.exists_letters.borrow_mut().insert(*l);
    }

    fn add_exists(&self, l: &char, p: &usize) {
        //println!("Restricting to words containing an {}" , l);

        let to_retain = self.with_letter(l).clone();
        self.candidates
            .borrow_mut()
            .retain(|s| to_retain.contains(s));

        //println!("Removing words with an {} at position {}" , l, p);
        let to_remove = self.with_letter_in_position(l, p).clone();
        self.candidates
            .borrow_mut()
            .retain(|s| !to_remove.contains(s));
        self.exists_letters.borrow_mut().insert(*l);
    }

    fn add_invalid(&self, l: &char) {
        if self.exists_letters.borrow().contains(l) {
            println!(
                "This letter {} has been hinted as existing already. Not removing it.",
                l
            );
        } else {
            //println!("Removing words with an {}" , l);
            let to_remove = self.with_letter(l).clone();
            self.candidates
                .borrow_mut()
                .retain(|s| !to_remove.contains(s));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_solver() {
        let words: HashSet<String> = vec![String::from("boudin"), String::from("blanc")]
            .into_iter()
            .collect();
        let solver = Solver::new(&words);
        let llen = |l| solver.with_letter(l).len();
        let lplen = |l, p| solver.with_letter_in_position(l, p).len();

        assert!(solver.first_candidate().is_some());
        assert_eq!(llen(&'b'), 2);
        assert_eq!(llen(&'o'), 1);
        assert_eq!(llen(&'u'), 1);
        assert_eq!(llen(&'d'), 1);
        assert_eq!(llen(&'i'), 1);
        assert_eq!(llen(&'n'), 2);

        assert_eq!(llen(&'l'), 1);
        assert_eq!(llen(&'a'), 1);
        assert_eq!(llen(&'c'), 1);
        assert_eq!(llen(&'z'), 0);

        assert_eq!(lplen(&'b', &0), 2);
        assert_eq!(lplen(&'o', &0), 0);
        assert_eq!(lplen(&'o', &1), 1);
        assert_eq!(lplen(&'u', &1), 0);
        assert_eq!(lplen(&'u', &2), 1);
    }
}
