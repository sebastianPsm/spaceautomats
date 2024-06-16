use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use crate::spaceautomat::Spaceautomat;

pub struct Physmodel {
    step_count: u64,
    width: u32,
    height: u32,
    rng: ChaCha8Rng,
    t: f64,
    m: f64,
}

impl Physmodel {
    pub fn new(width: u32, height: u32, seed: u64) -> Physmodel {
        Physmodel {
            step_count: 0,
            width: width,
            height: height,
            rng: ChaCha8Rng::seed_from_u64(seed),
            t: 1.0,
            m: 1.0,
        }
    }
    /// Get dimension (with and hight)
    pub fn get_dim(&self) -> (u32, u32) {
        (self.width, self.height)
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
                let power = f64::from(power) * if forward {1.0} else {-1.0};
                let a = power / self.m;
                let dir = automat.ship_hw.get_dir_rad();
                let pos = automat.ship_hw.get_pos();
                let vel = automat.ship_hw.get_velocity();

                let vel_new = (a * self.t * dir.sin() + vel.0, a * self.t * dir.cos() + vel.1);
                let mut pos_new = (vel.0 * self.t + pos.0 as f64, vel.1 * self.t + pos.1 as f64);
                pos_new.0 = if pos_new.0 > self.width as f64 { self.width as f64 } else { pos_new.0 };
                pos_new.1 = if pos_new.1 > self.height as f64 { self.height as f64 } else { pos_new.1 };

                automat.ship_hw.set_velocity(vel_new);
                automat.ship_hw.set_pos((pos_new.0 as u32, pos_new.1 as u32));
            }
        });
        self.step_count += 1;
    }
}