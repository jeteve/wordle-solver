use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::error;
use std::fmt;
use std::cell::RefCell;

lazy_static! {
    static ref EMPTYSET: HashSet<&'static String> = HashSet::new();
}

pub struct Solver<'a> {
    candidates: RefCell<HashSet<&'a String>>,
    exists_letters: RefCell<HashSet<char>>,
    by_letter: HashMap<char, HashSet<&'a String>>,
    by_letter_position: HashMap<(char, usize), HashSet<&'a String>>,
}

#[derive(Clone)]
pub struct FullHint {
    pub hint: Hint,
    pub c: char,
    pub p: usize,
}

#[derive(Clone, PartialEq)]
pub enum Hint {
    WellPlaced,
    Exists,
    Invalid,
}

#[derive(Debug)]
pub enum HintParseError {
    InvalidCode(String)
}
impl fmt::Display for HintParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot parse hint '{:?}'", self)
    }
}
impl error::Error for HintParseError {}

impl std::str::FromStr for Hint {
    type Err = HintParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Hint::Invalid),
            "e" => Ok(Hint::Exists),
            "g" => Ok(Hint::WellPlaced),
            _ => Err(HintParseError::InvalidCode(s.to_string())),
        }
    }
}

impl<'a> Solver<'a> {
    pub fn new(all_words: &HashSet<String>) -> Solver {
        let candidates = all_words.iter().collect::<HashSet<&String>>();
        let by_letter = all_words.iter().fold(HashMap::new(), |mut h, v| {
            for c in v.chars() {
                let entry = h.entry(c).or_insert_with(HashSet::new);
                entry.insert(v);
            }
            h
        });
        let by_letter_position = all_words.iter().fold(HashMap::new(), |mut h, v| {
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

    pub fn n_candidates(&self) -> usize {
        self.candidates.borrow().len()
    }

    pub fn first_candidate(&self) -> Option<&String> {
        return self.candidates.borrow().iter().next().map(|s| *s);
    }
    pub fn with_letter(&self, l: &char) -> &HashSet<&String> {
        self.by_letter.get(l).unwrap_or(&EMPTYSET)
    }
    pub fn with_letter_in_position(&self, l: &char, p: &usize) -> &HashSet<&String> {
        self.by_letter_position.get(&(*l, *p)).unwrap_or(&EMPTYSET)
    }

    // Ingest a bunch of hints together,
    // ensuring logical consistency between them.
    pub fn ingest_hints(&mut self, fhs: Vec<FullHint>) {
        let (valid, invalid): (Vec<_>, Vec<_>) = fhs.iter().partition(|&h| h.hint != Hint::Invalid);
        for fh in valid{
            self.add_full_hint(fh.clone());
        }
        for fh in invalid{
            self.add_full_hint(fh.clone());
        }

    }

    pub fn add_full_hint(&mut self, fh: FullHint) {
        self.add_hint(&fh.c, &fh.p, fh.hint)
    }
    pub fn add_hint(&mut self, l: &char, p: &usize, h: Hint) {
        match h {
            Hint::WellPlaced => self.add_well_placed(l, p),
            Hint::Exists => self.add_exists(l, p),
            Hint::Invalid => self.add_invalid(l),
        }
    }
    pub fn discard_word(&self, s: &str) {
        self.candidates.borrow_mut().remove(&s.to_string());
    }

    fn add_well_placed(&self, l: &char, p: &usize) {
        //println!("Restricting to words containing an {} at position {}" , l, p);
        let to_retain = self.with_letter_in_position(l, p).clone();
        self.candidates.borrow_mut().retain(|s| to_retain.contains(s));
        self.exists_letters.borrow_mut().insert(*l);
    }

    fn add_exists(&self, l: &char, p: &usize) {
        //println!("Restricting to words containing an {}" , l);

        let to_retain = self.with_letter(l).clone();
        self.candidates.borrow_mut().retain(|s| to_retain.contains(s));
       
        //println!("Removing words with an {} at position {}" , l, p);
        let to_remove = self.with_letter_in_position(l, p).clone();
        self.candidates.borrow_mut().retain(|s| !to_remove.contains(s));
        self.exists_letters.borrow_mut().insert(*l);
    }

    fn add_invalid(&self, l: &char) {
        if self.exists_letters.borrow().contains(l) {
            println!("This letter {} has been hinted as existing already. Not removing it.", l);
        }else{
            //println!("Removing words with an {}" , l);
            let to_remove = self.with_letter(l).clone();
            self.candidates.borrow_mut().retain(|s| !to_remove.contains(s));
        }
    }
}
