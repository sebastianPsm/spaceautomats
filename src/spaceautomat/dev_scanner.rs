use core::f64;

use super::{device::Device, Spaceautomat};

pub struct Scanner {
    slot_id: u8,
    enabled: bool,
    aperture_angle: f64,
    max_detection_distance: f64,
    heading: f64,
    sensitivity: f64
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            slot_id: 0,
            enabled: false,
            aperture_angle: 0.0,
            max_detection_distance: 0.0,
            heading: 0.0,
            sensitivity: 0.0
        }
    }
}

impl Device for Scanner {
    fn get_name() -> String {
        "scanner".to_string()
    }

    fn set_active(&mut self, slot_id: u8) {
        self.slot_id = slot_id;
    }

    fn write(&mut self, addr: u8, value: u8) {
        if self.slot_id == 0 {
            return;
        }

        match addr {
            0 => { self.enabled = (value & 0x01) == 1; }
            1 => { self.aperture_angle = value as f64 * 2.0 * std::f64::consts::PI / std::u8::MAX as f64 }
            2 => { self.max_detection_distance = 5000.0 * value as f64; }
            3 => { self.heading = value as f64 * 2.0 * std::f64::consts::PI / std::u8::MAX as f64 }
            4 => { self.sensitivity = value as f64 }
            _ => return
        }
    }

    fn read(&self, addr: u8) -> u8 {
        if self.slot_id == 0 {
            return 0;
        }

        match addr {
            5 => { return 0 }
            6 => { return 0 }
            7 => { return 0 }
            8 => { return 0 }
            9 => { return 0 }
            _ => return 0
        }
    }
}

impl Scanner {
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    pub fn get_aperture_angle(&self) -> f64 {
        self.aperture_angle
    }
    pub fn get_heading(&self) -> f64 {
        self.heading
    }
    pub fn get_max_detection_distance(&self) -> f64 {
        self.max_detection_distance
    }
    pub fn check(&self, ego: &Spaceautomat, all_positions: &Vec<(u32, u32)>) {
        let ego_pos = ego.ship_hw.get_pos();
        let ego_dir = ego.ship_hw.get_dir_rad();
        let ego_x = ego_pos.0 as f64;
        let ego_y = ego_pos.1 as f64;

        for pos in all_positions {
            let dx = pos.0 as f64 - ego_x;
            let dy = pos.1 as f64 - ego_y;
            if dx == 0.0 && dy == 0.0 { continue; }

            let absolut = (dy.atan2(dx) + 2.0*f64::consts::PI) % (2.0*f64::consts::PI);
            let relative = ego_dir - absolut;

            let relative_deg = relative / f64::consts::PI * 180.0;

            let x = relative_deg+1.0;
        }
    }
}