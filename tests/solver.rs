use wordle_solver::solver::{HintType, Solver};

use std::collections::HashSet;

#[test]
fn solver_hints() {
    let words: HashSet<String> = vec![
        String::from("class"),
        String::from("clock"),
        String::from("coach"),
        String::from("court"),
        String::from("carte"),
    ]
    .into_iter()
    .collect();
    let mut solver = Solver::new(&words);
    solver.discard_word("carte");
    assert_eq!(solver.n_candidates(), 4);
    solver.add_raw_hint(&'o', &5, HintType::Exists);
    assert_eq!(solver.n_candidates(), 3);

    solver.add_raw_hint(&'l', &1, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 1);

    solver.add_raw_hint(&'l', &1, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 1); // Buggy invalid hints are ignored.

    // The position does not really matter for an invalid hint
    solver.add_raw_hint(&'s', &0, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 1);
}
