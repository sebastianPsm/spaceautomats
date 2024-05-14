use spaceautomats_simulation::Simulation;

#[test]
fn load_and_release() {
    let mut sa = Simulation::new(5000,5000,1);
    
    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = "print('hello world!')".to_string();
    sa.load_automat(&automat);

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load tests/myautomat.lua");
    sa.load_automat(&automat);

    assert!(1 == sa.count_automats());

}
#[test]
fn load_20_space_automats_and_initialize() {
    let mut sa = Simulation::new(5000,5000,1);

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load test/myautomat.lua");

    for _idx in 1..11 {
        sa.load_automat(&automat);
    }
    assert!(10 == sa.count_automats());
    
    sa.init();
    assert!(10 == sa.count_initialized());
}
#[test]
fn load_space_automats_intialize_and_run_simulation() {
    let mut sa = Simulation::new(5000,5000,1);

    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load test/myautomat.lua");
    sa.load_automat(&automat);
    sa.load_automat(&automat);
    sa.load_automat(&automat);
    assert!(3 == sa.count_automats());

    sa.init();
    assert!(3 == sa.count_initialized());

    sa.step();
    sa.step();
    sa.step();

    let step_counts = sa.count_steps();
    assert!(step_counts[0] == 3);
    assert!(step_counts[1] == 3);
    assert!(step_counts[2] == 3);
}
#[test]
fn step() {
    let mut sa = Simulation::new(5000,5000,1);
    let automat = std::fs::read_to_string("tests/myautomat.lua").expect("Wasn't possible to load test/myautomat.lua");
    sa.load_automat(&automat);
    sa.init();
    sa.step();
}