pub mod spaceautomat;
mod physmodel;
mod plasma;

use std::path::PathBuf;

use spaceautomat::ReturnCode;
use crate::spaceautomat::Spaceautomat;
use crate::physmodel::Physmodel;
use crate::plasma::Plasma;

pub struct Simulation {
    automats: Vec<Spaceautomat>,
    plasmas: Vec<Plasma>,
    physmodel: Physmodel
}

impl Simulation {
    pub fn new(x: u32, y: u32, seed: u64) -> Simulation {
        Simulation {
            automats: Vec::new(),
            plasmas: Vec::new(),
            physmodel: Physmodel::new(x, y, seed)
        }
    }
    pub fn load_automat_by_file(&mut self, file_path: &PathBuf) {
        let code = std::fs::read_to_string(&file_path);
        if let Some(code) = code.ok() {
            self.load_automat(&code);
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
    /// Get plasmas
    pub fn get_plasmas(&self) -> Vec<&Plasma> {
        let mut ret: Vec<&Plasma> = vec![];
        for p in &self.plasmas {
            ret.push(&p);
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
        let mut id_cnt = 0;
        self.automats.iter_mut().for_each(|ele| { 
            ele.init();

            ele.set_id(id_cnt);
            id_cnt += 1;
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
        // Steps the physmodel with the automats
        self.physmodel.update(&mut self.automats, &mut self.plasmas);

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