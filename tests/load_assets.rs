use assets_manager::AssetCache;
use wordle_solver::*;

#[test]
fn load_asset(){
    let cache = AssetCache::new("assets");
    assert!(cache.is_ok());
    let words = load_words(&cache.unwrap(), "american-english");
    assert!(words.is_ok());
    assert_eq!(words.unwrap().len(), 93845);
}