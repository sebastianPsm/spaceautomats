mod spaceautomat;

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
    pub fn term(&mut self) {
        while !self.automats.is_empty() {
            self.automats.pop();
        }
    }
    pub fn load_automat(&mut self, code: &String) {
        let mut sa = Spaceautomat::new();
        sa.load_code(code);
        self.automats.push(sa);
    }
    pub fn count(&self) -> usize {
        return self.automats.len();
    }
}