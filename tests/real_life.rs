use assets_manager::AssetCache;
use wordle_solver::solver::*;
use wordle_solver::*;

#[test]
// Worlde from 13/01/2022
fn real_life() {
    let cache = AssetCache::new("assets");
    assert!(cache.is_ok());
    let words = load_words(&cache.unwrap(), "american-english", 5);
    assert!(words.is_ok());
    let actual_w = words.unwrap();
    let mut solver = Solver::new(&actual_w);
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );
    assert_eq!(solver.n_candidates(), 5905);

    solver.add_raw_hint(&'W', &0, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 5429);

    solver.add_raw_hint(&'H', &1, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 4691);

    solver.add_raw_hint(&'I', &2, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 3314);

    solver.add_raw_hint(&'P', &3, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 2797);

    solver.add_raw_hint(&'S', &4, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 1517);

    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_raw_hint(&'A', &0, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 144);

    solver.add_raw_hint(&'N', &1, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 96);

    solver.add_raw_hint(&'N', &2, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 96);

    solver.add_raw_hint(&'E', &3, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 19);

    solver.add_raw_hint(&'X', &4, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 19);

    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );
    solver.add_raw_hint(&'A', &0, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 19); // no move still well placed.

    solver.add_raw_hint(&'R', &1, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 6); // new invalid

    solver.add_raw_hint(&'M', &2, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 6); // new invalid but no change

    solver.add_raw_hint(&'E', &3, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 6); // same well placed as before

    solver.add_raw_hint(&'D', &4, HintType::Invalid);
    assert_eq!(solver.n_candidates(), 4); // new invalid

    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_raw_hint(&'A', &0, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 4); // existing well placed

    solver.add_raw_hint(&'B', &1, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 1); // new well placed

    solver.add_raw_hint(&'B', &2, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 1); // new well placed but no move

    solver.add_raw_hint(&'E', &3, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 1); // new well placed but no move

    solver.add_raw_hint(&'Y', &4, HintType::WellPlaced);
    assert_eq!(solver.n_candidates(), 1); // new well placed but no move.

    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    assert_eq!(solver.n_candidates(), 1);
    assert!(solver.first_candidate().unwrap().eq("ABBEY"));

    // Fake Hint, so we can check the number of candidates goes
    // down to zero.
    solver.add_raw_hint(&'E', &4, HintType::WellPlaced);
    assert_eq!(solver.first_candidate(), None);
}
