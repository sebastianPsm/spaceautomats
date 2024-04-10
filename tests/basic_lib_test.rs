use spaceautomats::{self, Spaceautomats};

#[test]
fn it_loads_and_releases() {
    let mut sa = Spaceautomats::new();
    
    let automat = "Automat 1".to_string();
    sa.load_automat(automat);

    let automat = "Automat 2".to_string();
    sa.load_automat(automat);

    assert!(2 == sa.count());

    sa.term();
}

