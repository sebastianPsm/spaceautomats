use core::f64;

use super::{device::Device, Spaceautomat};

#[derive(Debug, Copy, Clone)]
pub struct Detection {
    angle_relative: f64,
    angle_absolute: f64,
    distance: f64,
}
impl Detection {
    pub fn get_angle_r(&self) -> f64 {
        self.angle_relative
    }
    pub fn get_angle_a(&self) -> f64 {
        self.angle_absolute
    }
    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}

pub struct Scanner {
    slot_id: u8,
    enabled: bool,
    aperture_angle: f64,
    max_detection_distance: f64,
    heading: f64,
    sensitivity: f64,
    detections: Vec<Detection>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            slot_id: 0,
            enabled: false,
            aperture_angle: 0.0,
            max_detection_distance: 0.0,
            heading: 0.0,
            sensitivity: 0.0,
            detections: vec![],
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
            2 => { self.max_detection_distance = 1000.0 * value as f64; }
            3 => { self.heading = value as f64 * 2.0 * std::f64::consts::PI / std::u8::MAX as f64 }
            4 => { self.sensitivity = value as f64 }
            _ => return
        }
    }

    fn read(&self, addr: u8) -> u8 {
        if self.slot_id == 0 {
            return 0;
        }

        let detects = self.get_detections();

        let addr = addr as usize;
        match addr {
            1 => { return (self.aperture_angle / 2.0 / std::f64::consts::PI * std::u8::MAX as f64) as u8; }
            3 => { return (self.heading / 2.0 / std::f64::consts::PI * std::u8::MAX as f64) as u8; }
            5 => { return detects.len() as u8 }
            6 | 8 | 10 | 12 | 14 => {
                let idx = (addr-4) / 2;
                if detects.len() < idx { return 0; }

                let distance = (detects[idx-1].get_distance() / self.max_detection_distance * 255.0) as u8;
                return distance;
            }
            7 | 9 | 11 | 13 | 15 => {
                let idx = (addr-5) / 2;
                if detects.len() < idx { return 0; }

                let angle_norm = 0.5 + detects[idx-1].get_angle_r() / self.aperture_angle;
                let angle_norm_u8 = (angle_norm*255.0).ceil() as u8;

                return angle_norm_u8;
            }
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
    pub fn set_detections(&mut self, detections: Vec<Detection>) {
        self.detections = detections;
    }
    pub fn get_detections(&self) -> Vec<Detection> {
        self.detections.clone()
    }
    pub fn check(&self, ego: &Spaceautomat, all_positions: &Vec<(u32, u32)>) -> Vec<Detection> {
        let ego_pos = ego.ship_hw.get_pos();
        let ego_dir = ego.ship_hw.get_dir_rad();
        let aperture_angle = ego.ship_hw.scanner.get_aperture_angle();
        let aperture_heading = 2.0*f64::consts::PI + ego.ship_hw.scanner.get_heading();
        let aperture_angle_1 = aperture_heading - aperture_angle/2.0;
        let aperture_angle_2 = aperture_angle_1 + aperture_angle;

        let mut result: Vec<Detection> = vec![];

        let mut nearest = f64::INFINITY;
        for pos in all_positions {
            if pos.0 == ego_pos.0 && pos.1 == ego_pos.1 { continue; }
            let dx = pos.0 as f64 - ego_pos.0 as f64;
            let dy = pos.1 as f64 - ego_pos.1 as f64;

            let absolut = (dy.atan2(dx) + 2.0*f64::consts::PI) % (2.0*f64::consts::PI);
            let relative = ego_dir - absolut;
            let relative_2pi = relative + 2.0*f64::consts::PI;
            let distance = (dx.powi(2)+dy.powi(2)).sqrt();

            if distance > self.max_detection_distance { continue; }
            if !(aperture_angle_1 <= relative_2pi && relative_2pi <= aperture_angle_2) { continue; }

            let mut index = result.len();
            if distance < nearest {
                nearest = distance;
                index = 0;
            }
            result.insert(index, Detection {
                angle_relative: relative,
                angle_absolute: absolut,
                distance: distance,
            });
        }

        result.truncate(5);

        result
    }
}