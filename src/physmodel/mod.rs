use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::spaceautomat::Spaceautomat;

pub struct Physmodel {
    step_count: u64,
    x: u64,
    y: u64,
    rng: ChaCha8Rng
}

impl Physmodel {
    pub fn new(x: u64, y: u64, seed: u64) -> Physmodel {
        Physmodel {
            step_count: 0,
            x: x,
            y: y,
            rng: ChaCha8Rng::seed_from_u64(seed)
        }
    }
    pub fn init(&mut self, automats: &mut Vec<Spaceautomat>) {
        automats.iter_mut().for_each(|automat| {
            let x = self.rng.gen_range(0..self.x);
            let y = self.rng.gen_range(0..self.y);
            let dir = self.rng.gen_range(0..3599);

            automat.set_pos((x,y));
            automat.set_dir(dir);
        });
    }
    pub fn update(&mut self, automats: &mut Vec<Spaceautomat>) {
        
        self.step_count += 1;
    }
}