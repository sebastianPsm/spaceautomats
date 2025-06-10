use std::ops::Mul;

use super::device::Device;

pub struct ReactionWheel {
    slot_id: u8,
    enabled: bool,
    torque_dir: bool, // true: counter-clock
    torque: u16,
    angular_velo: [u8; 4],
    angular_velo_counterclock: bool,
}

impl ReactionWheel {
    pub fn new() -> ReactionWheel {
        ReactionWheel {
            slot_id: 0,
            enabled: false,
            torque_dir: true,
            torque: 0,
            angular_velo: [0, 0, 0, 0],
            angular_velo_counterclock: false
        }
    }
}

impl Device for ReactionWheel {
    fn get_name() -> String {
        "reaction wheel".to_string()
    }
    fn set_active(&mut self, slot_id: u8) {
        self.slot_id = slot_id;
    }
    fn write(&mut self, addr: u8, value: u8) {
        if self.slot_id == 0 {
            return;
        }

        match addr {
            0 => { 
                self.enabled = (value & 0x01) == 1;
                self.torque_dir = if (value & 0x02) == 2 {true} else {false};
            }
            1 => { self.torque = (self.torque as u16 & 0xFF00) | (value as u16 & 0x00FF); }
            2 => { self.torque = (self.torque as u16 & 0x00FF) | ((value as u16) << 8) & 0xFF00; }
            _ => return
        }
    }
    fn read(&self, addr: u8) -> u8 {
        if self.slot_id == 0 {
            return 0;
        }

        match addr {
            2 => { return self.angular_velo_counterclock as u8; }
            3 => { return self.angular_velo[0]; }
            4 => { return self.angular_velo[1]; }
            5 => { return self.angular_velo[2]; }
            6 => { return self.angular_velo[3]; }
            _ => return 0
        }
    }
}
impl ReactionWheel {
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    pub fn get_torque(&self) -> f64 {
        if self.torque_dir { self.torque as f64 } else { -(self.torque as f64) }
    }
    pub fn set_angular_velocity(&mut self, angular_velo: f64) {
        self.angular_velo_counterclock = if angular_velo >= 0.0 { true } else { false };        
        self.angular_velo = ((angular_velo as f32).abs().mul(1000000.0).clamp(0.0, 1000000.0) as u32).to_le_bytes(); // in µrad/step
    }
}