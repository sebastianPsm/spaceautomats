use mlua::{UserData, UserDataMethods};

use super::device::Device;
use super::dev_propulsion::Propulsion;

pub struct Ship {
    pub propulsion: Propulsion,

    pos: (u64, u64), // (x,y)
    velocity: (f32, f32),
    dir: u16 // direction in deg*10 (0..3599)
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            propulsion: Propulsion::new(),

            pos: (0, 0),
            velocity: (0.0, 0.0),
            dir: 0,
        }
    }
    /// Set the position
    pub fn set_pos(&mut self, pos: (u64, u64)) {
        self.pos = pos;
    }
    /// Set the direction
    pub fn set_dir(&mut self, dir: u16) {
        self.dir = dir;
    }
}
impl UserData for Ship {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("slot", |_, ship, (slot_id, devicestr): (u8, String)| {
            if devicestr.eq("propulsion") {
                ship.propulsion.set_slot(slot_id);
            }
            Ok(())
        });
        methods.add_method_mut("write", |_, ship, (addr, value):(u32, u8)| {
            ship.propulsion.write(addr, value);
            Ok(())
        });
        methods.add_method("read", |_, ship, addr: u32|{
            let value = ship.propulsion.read(addr);
            Ok(value)
        })
    }
}