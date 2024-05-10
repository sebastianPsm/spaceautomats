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
    pub fn load_automat(&mut self, code: &String) {
        let mut sa = Spaceautomat::new();
        let rc = sa.load_code(code);
        if matches!(rc, ReturnCode::Ok) {
            self.automats.push(sa);
        }
    }
    pub fn count_automats(&self) -> usize {
        return self.automats.len();
    }
    pub fn init(&mut self) {
        self.automats.iter_mut().for_each(|ele| { 
            ele.init(); 
        });
    }
    pub fn count_initialized(&self) -> usize {
        let mut result = 0;

        self.automats.iter().for_each(|ele| {
            if ele.is_initialized() {
                result += 1;
            }
        });

        return result;
    }
    pub fn step(&mut self) {
        self.automats.iter_mut().for_each(|ele| {
            if !ele.is_initialized() { return; }
            ele.step();
        });
    }
    pub fn count_steps(&self) -> Vec<u64> {
        let mut result: Vec<u64> = Vec::new();
        
        self.automats.iter().for_each(|ele| {
            result.push(ele.get_step_count());
        });

        return result;
    }
}