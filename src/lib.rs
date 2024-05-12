mod spaceautomat;

use spaceautomat::ReturnCode;
use crate::spaceautomat::Spaceautomat;

pub struct Simulation {
    automats: Vec<Spaceautomat>
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            automats: Vec::new()
        }
    }
    /// Loads the Lua code for a automat
    /// # Example
    ///
    /// ```Lua
    ///-- The init()-function is called once before every simulation
    ///-- Use init() to configure your space automat.
    ///function init(training)
    ///    ship:slot(0, "propulsion")
    ///    ship:slot(1, "reaction wheels")
    ///end
    ///
    ///-- The run()-function is called in every simulation step
    ///function run(step)
    ///    ship:write(1024, 3);
    ///    ship:write(1025, 255);
    ///end
    /// ```
    pub fn load_automat(&mut self, code: &String) {
        let mut sa = Spaceautomat::new();
        let rc = sa.load_code(code);
        if matches!(rc, ReturnCode::Ok) {
            self.automats.push(sa);
        }
    }
    /// Counts the loaded automats (automats with invalid Lua syntax are not loaded)
    pub fn count_automats(&self) -> usize {
        return self.automats.len();
    }
    /// Calls the init()-function from all automats and initializes the simulation
    pub fn init(&mut self) {
        self.automats.iter_mut().for_each(|ele| { 
            ele.init(); 
        });
    }
    /// Counts the initializes automats
    pub fn count_initialized(&self) -> usize {
        let mut result = 0;

        self.automats.iter().for_each(|ele| {
            if ele.is_initialized() {
                result += 1;
            }
        });

        return result;
    }
    /// Calls the run()-function from all automats to perform one simulation step
    pub fn step(&mut self) {
        // Call step for each automat --> calls run()-function from each automat
        self.automats.iter_mut().for_each(|ele| {
            if !ele.is_initialized() { return; }
            ele.step();
        });

    }
    /// Provides a vector of simulation step counters for each automat
    pub fn count_steps(&self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        
        self.automats.iter().for_each(|ele| {
            result.push(ele.get_step_count());
        });

        return result;
    }
}