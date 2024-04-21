use spaceautomats_simulation::{Simulation};

#[test]
fn it_loads_and_releases() {
    let mut sa = Simulation::new();
    
    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load tests/myautomat.lua");
    sa.load_automat(&automat);

    assert!(1 == sa.count());

    sa.term();
}

#[test]
fn it_loads_20_space_automats_and_starts_the_simulation() {
    let mut sa = Simulation::new();

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load test/myautomat.lua");

    for _idx in 1..11 {
        sa.load_automat(&automat);
    }
    assert!(10 == sa.count());

    

    sa.term();
}