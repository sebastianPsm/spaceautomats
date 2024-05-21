use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::spaceautomat::Spaceautomat;

pub struct Physmodel {
    step_count: u64,
    width: u64,
    height: u64,
    rng: ChaCha8Rng
}

impl Physmodel {
    pub fn new(width: u64, height: u64, seed: u64) -> Physmodel {
        Physmodel {
            step_count: 0,
            width: width,
            height: height,
            rng: ChaCha8Rng::seed_from_u64(seed)
        }
    }
    pub fn init(&mut self, automats: &mut Vec<Spaceautomat>) {
        automats.iter_mut().for_each(|automat| {
            let x = self.rng.gen_range(0..self.width);
            let y = self.rng.gen_range(0..self.height);
            let dir = self.rng.gen_range(0..3599);

            automat.ship_hw.set_pos((x,y));
            automat.ship_hw.set_dir(dir);
        });
    }
    pub fn update(&mut self, automats: &mut Vec<Spaceautomat>) {
        automats.iter_mut().for_each(|automat| {
            /*
             * Propulsion
             */
            let fuel = automat.ship_hw.propulsion.get_fuel();
            let power = automat.ship_hw.propulsion.get_power();
            let forward = automat.ship_hw.propulsion.get_forward();
            if power > 0 && fuel >= u32::from(power) {
                automat.ship_hw.propulsion.set_fuel(fuel-u32::from(power));
                let power = i32::from(power) * if forward {1} else {-1};
                
            }
        });
        self.step_count += 1;
    }
}