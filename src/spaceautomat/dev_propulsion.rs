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
    fn set_slot(&mut self, slot_id: u8) {
        self.slot_id = slot_id;
    }
    fn get_slot(&self) -> u8 {
        return self.slot_id;
    }
    fn write(&mut self, addr: u32, value: u8) {
        
    }
    fn read(&self, addr: u32) -> u8 {
        return 0;
    }
}
