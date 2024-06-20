use super::device::Device;

pub struct Propulsion {
    slot_id: u8,
    enabled: bool,
    forward: bool,
    fuel: u32,
    power: u8,
}

impl Propulsion {
    pub fn new() -> Propulsion {
        Propulsion {
            slot_id: 0,
            enabled: false,
            forward: true,
            fuel: 0,
            power: 0,
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
            0 => { return (self.fuel & 0x000000FF >>  0) as u8 }
            1 => { return (self.fuel & 0x0000FF00 >>  8) as u8 }
            2 => { return (self.fuel & 0x00FF0000 >> 16) as u8 }
            3 => { return (self.fuel & 0xFF000000 >> 24) as u8 }
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
}