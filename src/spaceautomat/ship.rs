use mlua::{UserData, UserDataMethods};

use super::device::Device;
use super::dev_propulsion::Propulsion;

pub struct Ship {
    pub propulsion: Propulsion,

    pos: (u32, u32), // (x,y)
    velocity: (f64, f64),
    dir: u16, // direction in deg*10 (0..3599)
    name: String,
    health: u16
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            propulsion: Propulsion::new(),

            pos: (0, 0),
            velocity: (0.0, 0.0),
            dir: 0,
            name: "MyShip".to_string(),
            health: u16::MAX,
        }
    }
    /// Get position
    pub fn get_pos(&self) -> (u32, u32) {
        self.pos
    }
    /// Set position
    pub fn set_pos(&mut self, pos: (u32, u32)) {
        self.pos = pos;
    }
    /// Get velocity
    pub fn get_velocity(&self) -> (f64, f64) {
        self.velocity
    }
    /// Set velocity
    pub fn set_velocity(&mut self, value: (f64, f64)) {
        self.velocity = value;
    }
    /// Get direction
    pub fn get_dir(&self) -> u16 {
        self.dir
    }
    /// Set direction
    pub fn set_dir(&mut self, dir: u16) {
        self.dir = dir;
    }
    /// Set name
    pub fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }
    /// Get name
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    /// Get health
    pub fn get_health(&self) -> u16 {
        self.health
    }
}
impl UserData for Ship {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("name", |_, ship, name: String| {
            ship.set_name(&name);
            Ok(())
        });
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