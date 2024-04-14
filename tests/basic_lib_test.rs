use spaceautomats::{self, Simulation};

#[test]
fn it_loads_and_releases() {
    let mut sa = Simulation::new();
    
    let automat = "print('hello world!')".to_string();
    sa.load_automat(automat);

    let automat = "print('hello world!')".to_string();
    sa.load_automat(automat);

    assert!(2 == sa.count());

    sa.term();
}

