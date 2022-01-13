use assets_manager::AssetCache;
use wordle_solver::solver::*;
use wordle_solver::*;

#[test]
// Worlde from 13/01/2022
fn real_life() {
    let cache = AssetCache::new("assets");
    assert!(cache.is_ok());
    let words = load_words(&cache.unwrap(), "american-english");
    assert!(words.is_ok());
    let actual_w = words.unwrap();
    let mut solver = Solver::new(&actual_w);
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_hint(&'W', &0, Hint::Invalid);
    solver.add_hint(&'H', &1, Hint::Invalid);
    solver.add_hint(&'I', &2, Hint::Invalid);
    solver.add_hint(&'P', &3, Hint::Invalid);
    solver.add_hint(&'S', &4, Hint::Invalid);
    solver.refresh_candidates();
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_hint(&'A', &0, Hint::WellPlaced);
    solver.add_hint(&'N', &1, Hint::Invalid);
    solver.add_hint(&'N', &2, Hint::Invalid);
    solver.add_hint(&'E', &3, Hint::WellPlaced);
    solver.add_hint(&'X', &4, Hint::Invalid);

    solver.refresh_candidates();
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_hint(&'A', &0, Hint::WellPlaced);
    solver.add_hint(&'R', &1, Hint::Invalid);
    solver.add_hint(&'M', &2, Hint::Invalid);
    solver.add_hint(&'E', &3, Hint::WellPlaced);
    solver.add_hint(&'D', &4, Hint::Invalid);

    solver.refresh_candidates();
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    solver.add_hint(&'A', &0, Hint::WellPlaced);
    solver.add_hint(&'B', &1, Hint::WellPlaced);
    solver.add_hint(&'B', &2, Hint::WellPlaced);
    solver.add_hint(&'E', &3, Hint::WellPlaced);
    solver.add_hint(&'Y', &4, Hint::WellPlaced);

    solver.refresh_candidates();
    println!(
        "Candidate: {:?} (out of {})",
        solver.first_candidate(),
        solver.n_candidates()
    );

    assert!(solver.first_candidate().unwrap().eq("ABBEY"));
}
