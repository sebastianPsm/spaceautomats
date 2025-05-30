use std::{f64, ops::Mul};

use super::device::Device;

pub struct Propulsion {
    slot_id: u8,
    enabled: bool,
    forward: bool,
    fuel: u32,
    power: u8,
    velocity_abs: [u8; 2],
    velocity_dir: [u8; 4],
    heading: [u8; 4]
}

impl Propulsion {
    pub fn new() -> Propulsion {
        Propulsion {
            slot_id: 0,
            enabled: false,
            forward: true,
            fuel: 0,
            power: 0,
            velocity_abs: [0, 0],
            velocity_dir: [0, 0, 0, 0],
            heading: [0, 0, 0, 0]
        }
    }
}

impl Device for Propulsion {
    fn get_name() -> String {
        "propulsion".to_string()
    }
    fn set_active(&mut self, slot_id: u8) {
        self.slot_id = slot_id;
        self.fuel = if self.slot_id != 0 { u32::MAX } else { 0 };
    }
    fn write(&mut self, addr: u8, value: u8) {
        if self.slot_id == 0 {
            return;
        }

        match addr {
            0 => {
                self.enabled = (value & 0x01) == 1;
                self.forward = (value & 0x02) == 2;
            }
            1 => { self.power = value }
            _ => return
        }
    }
    fn read(&self, addr: u8) -> u8 {
        if self.slot_id == 0 {
            return 0;
        }

        match addr {
            2 => { return (self.fuel & 0x000000FF >>  0) as u8 }
            3 => { return (self.fuel & 0x0000FF00 >>  8) as u8 }
            4 => { return (self.fuel & 0x00FF0000 >> 16) as u8 }
            5 => { return (self.fuel & 0xFF000000 >> 24) as u8 }

            6 => { return self.velocity_abs[0] }
            7 => { return self.velocity_abs[1] }

            8 => { return self.velocity_dir[0] }
            9 => { return self.velocity_dir[1] }
            10 => { return self.velocity_dir[2] }
            11 => { return self.velocity_dir[3] }

            12 => { return self.heading[0] }
            13 => { return self.heading[1] }
            14 => { return self.heading[2] }
            15 => { return self.heading[3] }
            _ => return 0
        }
    }
}
impl Propulsion {
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    pub fn get_fuel(&self) -> u32 {
        self.fuel
    }
    pub fn set_fuel(&mut self, value: u32) {
        self.fuel = value;
    }
    pub fn get_power(&self) -> u8 {
        self.power
    }
    pub fn get_forward(&self) -> bool {
        self.forward
    }
    pub fn set_velocity(&mut self, velocity: (f64, f64), direction: f64) {
        self.velocity_dir = (velocity.0.atan2(velocity.1).mul(1000000.0).clamp(0.0, f64::consts::PI*2.0*1000000.0) as u32).to_le_bytes(); // in µrad
        let velocity = (velocity.0.powi(2) + velocity.1.powi(2)).sqrt();
        self.velocity_abs = (velocity.clamp(i16::MIN as f64, i16::MAX as f64) as i16).to_le_bytes();        
        self.heading = (direction.mul(1000000.0).clamp(0.0, f64::consts::PI*2.0*1000000.0) as u32).to_le_bytes(); // in µrad
    }
}