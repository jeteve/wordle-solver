use wordle_solver::solver::{Hint, Solver};

use std::collections::HashSet;

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

#[test]
fn solver_hints() {
    let words: HashSet<String> = vec![
        String::from("class"),
        String::from("clock"),
        String::from("coach"),
        String::from("court"),
    ]
    .into_iter()
    .collect();
    let mut solver = Solver::new(&words);
    assert_eq!(solver.n_candidates(), 4);
    solver.add_hint(&'o', &5, Hint::Exists);
    solver.refresh_candidates();
    assert_eq!(solver.n_candidates(), 3);

    solver.add_hint(&'l', &1, Hint::WellPlaced);
    solver.refresh_candidates();
    assert_eq!(solver.n_candidates(), 1);

    // The position does not really matter for an invalid hint
    solver.add_hint(&'s', &0, Hint::Invalid);
    solver.refresh_candidates();
    assert_eq!(solver.n_candidates(), 1);
}
