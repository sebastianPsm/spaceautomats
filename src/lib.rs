pub mod spaceautomat;
mod physmodel;

use spaceautomat::ReturnCode;
use crate::spaceautomat::Spaceautomat;
use crate::physmodel::Physmodel;

pub struct Simulation {
    automats: Vec<Spaceautomat>,
    physmodel: Physmodel
}

impl Simulation {
    pub fn new(x: u32, y: u32, seed: u64) -> Simulation {
        Simulation {
            automats: Vec::new(),
            physmodel: Physmodel::new(x, y, seed)
        }
    }
    pub fn load_automat(&mut self, code: &str) {
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
    /// Get automats
    pub fn get_automats(&self) -> Vec<&Spaceautomat> {
        let mut ret: Vec<&Spaceautomat> = vec![];
        for sa in &self.automats {
            ret.push(&sa);
        }
        ret
    }
    /// Get physmodel
    pub fn get_physmodel(&self) -> &Physmodel {
        &self.physmodel
    }
    /// Calls the init()-function from all automats and initializes the simulation
    pub fn init(&mut self) {
        // Initialize each automat --> calls init()-function from each automat
        self.automats.iter_mut().for_each(|ele| { 
            ele.init(); 
        });

        // Initialize the physmodel with the automats
        self.physmodel.init(&mut self.automats);
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
        
        // Steps the physmodel with the automats
        self.physmodel.update(&mut self.automats);
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