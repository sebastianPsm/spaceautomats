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
            result += if ele.is_initialized() { 1 } else { 0 };
        });

        return result;
    }

}