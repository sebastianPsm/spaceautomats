use super::device::Device;

pub struct Plasmacannon {
    slot_id: u8,
    enabled: bool,
    last_shot: u64
}

impl Plasmacannon {
    pub fn new() -> Plasmacannon {
        Plasmacannon {
            slot_id: 0,
            enabled: false,
            last_shot: 0
        }
    }
}

impl Device for Plasmacannon {
    fn get_name() -> String {
        "plasma cannon".to_string()
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
            }
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
impl Plasmacannon {
    pub fn get_enabled(&self) -> bool {
        self.enabled
    }
    pub fn set_last_shot(&mut self, step: u64) {
        self.last_shot = step
    }
    pub fn get_last_shot(&self) -> u64 {
        self.last_shot
    }
}