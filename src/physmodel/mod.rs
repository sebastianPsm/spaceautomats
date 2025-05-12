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
    i: f64,
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
            i: 1.0,
        }
    }
    /// Get dimension (width and hight)
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
        let mut all_positions = Vec::new();
        for automat in automats.iter() {
            all_positions.push(automat.ship_hw.get_pos());
        }

        for automat in automats.iter_mut() {
            let mut fuel: u32 = automat.ship_hw.propulsion.get_fuel();

            /*
             * Scanner (scan before move)
             */
            if automat.ship_hw.scanner.get_enabled() {
                fuel = fuel - 1;
                let d = automat.ship_hw.scanner.check(automat, &all_positions);
                automat.ship_hw.scanner.set_detections(d);
            }

            /*
             * Propulsion
             */
            let propulsion_enabled = automat.ship_hw.propulsion.get_enabled();
            let mut power: f64 = 0.0;

            if propulsion_enabled {
                fuel = fuel - 1;
                let power_value = automat.ship_hw.propulsion.get_power();    
                if power_value > 0 && fuel >= u32::from(power_value) {
                    fuel = fuel-u32::from(power_value);
                    automat.ship_hw.propulsion.set_fuel(fuel);

                    let forward = automat.ship_hw.propulsion.get_forward();                    
                    power = f64::from(power_value) * if forward {1.0} else {-1.0};
                }
            }

            /*
             * Reaction wheel
             */
            let reaction_wheel_enabled = automat.ship_hw.reaction_wheel.get_enabled();
            let mut ang_accel: f64 = 0.0;

            if reaction_wheel_enabled {
                fuel = fuel - 1;
                let power_value = automat.ship_hw.reaction_wheel.get_power();
                if power_value > 0 && fuel >= u32::from(power_value) {
                    fuel = fuel-u32::from(power_value);
                    automat.ship_hw.propulsion.set_fuel(fuel);

                    let counterclock = automat.ship_hw.reaction_wheel.get_counterclock();
                    ang_accel = f64::from(power_value) * if counterclock {1.0} else {-1.0} / 10000.0;
                }
                
            }

            let m = ang_accel / self.i;
            let dir: f64 = automat.ship_hw.get_dir_rad();
            let angular_velo = automat.ship_hw.get_angular_velocity_rad();

            let angular_velo_new = angular_velo + m*self.t;
            let dir_new = angular_velo_new * self.t + dir;           

            let s = (automat.ship_hw.get_pos().0 as f64, automat.ship_hw.get_pos().1  as f64);
            let v = automat.ship_hw.get_speed();
            let a = (power / self.m * dir_new.cos(), power / self.m * dir_new.sin());            

            let mut s_new = (s.0 + v.0 * self.t + a.0 * self.t*self.t, 
                                         s.1 + v.1 * self.t + a.1 * self.t*self.t);
            
            if reaction_wheel_enabled {
                automat.ship_hw.reaction_wheel.set_angular_velocity(angular_velo_new);
            }

            /*
             * Boundary
             */             
            s_new.0 = if s_new.0 > self.width.into() { self.width.into() } else { s_new.0 };
            s_new.0 = if s_new.0 < 0.0 { 0.0 } else { s_new.0 };
            s_new.1 = if s_new.1 > self.height.into() { self.height.into() } else { s_new.1 };
            s_new.1 = if s_new.1 < 0.0 { 0.0 } else { s_new.1 };

            let v_new = (s_new.0 - s.0 / self.t, s_new.1 - s.1 / self.t);

            automat.ship_hw.set_angular_velocity_rad(angular_velo_new);
            automat.ship_hw.set_dir_rad(dir_new);
            automat.ship_hw.set_speed(v_new);
            automat.ship_hw.set_pos((s_new.0 as u32, s_new.1 as u32));

            
        }
        self.step_count += 1;
    }
}