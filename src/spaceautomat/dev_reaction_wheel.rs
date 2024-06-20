use super::device::Device;

pub struct Reaction_wheel {
    slot_id: u8,
    enabled: bool,
    counterclock: bool,
    power: u8,
}

impl Reaction_wheel {
    pub fn new() -> Reaction_wheel {
        Reaction_wheel {
            slot_id: 0,
            enabled: false,
            counterclock: false,
            power: 0,
        }
    }
}

impl Device for Reaction_wheel {
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
                self.counterclock = (value & 0x02) == 2;
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
            _ => return 0
        }
    }
}
impl Reaction_wheel {
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    pub fn get_power(&self) -> u8 {
        self.power
    }
    pub fn get_counterclock(&self) -> bool {
        self.counterclock
    }
}