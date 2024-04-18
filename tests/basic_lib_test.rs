use spaceautomats_simulation::{Simulation};

#[test]
fn it_loads_and_releases() {
    let mut sa = Simulation::new();
    
    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load tests/myautomats.lua");
    sa.load_automat(&automat);

    assert!(3 == sa.count());

    sa.term();
}

