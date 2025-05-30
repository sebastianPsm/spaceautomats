pub(crate) mod spaceobject;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use spaceobject::Spaceobject;

use crate::{plasma::Plasma, spaceautomat::Spaceautomat};

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

            automat.ship_hw.object.set_pos((x,y));
            automat.ship_hw.object.set_dir(dir);
        });
    }
    pub fn update(&mut self, automats: &mut Vec<Spaceautomat>, plasmas: &mut Vec<Plasma>) {
        /*
         * Automats
         */
        let mut all_positions = Vec::new();
        for automat in automats.iter() {
            all_positions.push(automat.ship_hw.object.get_pos());
        }

        for automat in automats.iter_mut() {
            let mut fuel: u32 = automat.ship_hw.propulsion.get_fuel();

            /*
             * in Scanner (scan before move)
             */
            if automat.ship_hw.scanner.get_enabled() {
                fuel = fuel - 1;
                let d = automat.ship_hw.scanner.check(automat, &all_positions);
                automat.ship_hw.scanner.set_detections(d);
            }

            /*
             * in propulsion
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
             * in reaction wheel
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

            /*
             * in plasma cannon
             */
            if automat.ship_hw.plasmacannon.get_enabled() && (automat.ship_hw.plasmacannon.get_last_shot() + 3) < self.step_count  {
                automat.ship_hw.plasmacannon.set_last_shot(self.step_count);
                let mut p = Plasma::new(automat.get_id());
                let d = automat.ship_hw.object.get_dir_rad() + self.rng.gen_range(-0.2 .. 0.2);
                let mut s = automat.ship_hw.object.get_speed();
                s.0 += 10000.0 * d.cos();
                s.1 += 10000.0 * d.sin();
                p.object.set_pos(automat.ship_hw.object.get_pos());
                p.object.set_speed(s);
                p.object.set_dir_rad(d);
                plasmas.push(p); // spawn new plasma
            }

            /*
             * process kinematics
             */
            let (angular_velo_new, dir_new, v_new) = self.kinematics(&mut automat.ship_hw.object, power, ang_accel);
            
            /*
             * out reaction wheel
             */
            if reaction_wheel_enabled {
                automat.ship_hw.reaction_wheel.set_angular_velocity(angular_velo_new);
            }

            /*
             * out propulsion
             */
            if propulsion_enabled {
                automat.ship_hw.propulsion.set_velocity(v_new, dir_new);
            }
            
        }

        /*
         * Plasma
         */
        let mut plasmas_new: Vec<Plasma> = vec![];
        for plasma in plasmas.into_iter() {
            if plasma.is_on_boundary(self.width, self.height) {
                continue;
            }

            let mut collision = false;
            for automat in automats.iter_mut() {
                if automat.get_id() == plasma.get_source_id() {
                    continue;
                }

                if automat.ship_hw.object.check_collision(&plasma.object) {
                    collision = true;
                    automat.ship_hw.apply_damage(100);
                    break;
                }
            }
            if collision {
                continue;
            }

            let (_, dir_new, v_new) = self.kinematics(&mut plasma.object, 0.0, 0.0);
            plasma.object.set_dir_rad(dir_new);
            plasma.object.set_speed(v_new);
            plasmas_new.push(plasma.clone());
        }
        *plasmas = plasmas_new;

        self.step_count += 1;
    }

fn kinematics(&self, object: &mut Spaceobject, power: f64, ang_accel: f64) -> (f64, f64, (f64, f64)) {
        let m = ang_accel / self.i;
        let dir: f64 = object.get_dir_rad();
        let angular_velo = object.get_angular_velocity_rad();

        let angular_velo_new = angular_velo + m*self.t;
        let dir_new = angular_velo_new * self.t + dir;

        let s = (object.get_pos().0 as f64, object.get_pos().1  as f64);
        let v = object.get_speed();
        let a = (power / self.m * dir_new.cos(), power / self.m * dir_new.sin());

        let mut s_new = (s.0 + v.0 * self.t + a.0 * self.t*self.t, 
                                     s.1 + v.1 * self.t + a.1 * self.t*self.t);

        /*
         * Boundary
         */
        let r = object.get_size() as f64;
        s_new.0 = if s_new.0 > self.width as f64 - r { self.width as f64 - r } else { s_new.0 };
        s_new.0 = if s_new.0 < r { r } else { s_new.0 };
        s_new.1 = if s_new.1 > self.height as f64 - r { self.height as f64 - r } else { s_new.1 };
        s_new.1 = if s_new.1 < r { r } else { s_new.1 };

        let v_new = (s_new.0 - s.0 / self.t, s_new.1 - s.1 / self.t);

        object.set_angular_velocity_rad(angular_velo_new);
        object.set_dir_rad(dir_new);
        object.set_speed(v_new);
        object.set_pos((s_new.0 as u32, s_new.1 as u32));
        (angular_velo_new, dir_new, v_new)
    }
}