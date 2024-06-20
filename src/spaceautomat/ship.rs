use std::collections::HashMap;

use mlua::{UserData, UserDataMethods};

use super::dev_reaction_wheel::Reaction_wheel;
use super::device::Device;
use super::dev_propulsion::Propulsion;

enum DeviceEnum {
    Propulsion,
    ReactionWheel,
}

pub struct Ship {
    pub propulsion: Propulsion,
    pub reaction_wheel: Reaction_wheel,
    
    device_map: HashMap<u8, DeviceEnum>,

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
            reaction_wheel: Reaction_wheel::new(),
            device_map: HashMap::new(),

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
    /// Get direction [rad]
    pub fn get_dir_rad(&self) -> f64 {
        (self.dir as f64) / 10.0 / 180.0 * std::f64::consts::PI
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
            if slot_id == 0 {
                return Ok(())
            }

            if Propulsion::get_name().eq(&devicestr) {
                ship.propulsion.set_active(slot_id);
                ship.device_map.insert(slot_id, DeviceEnum::Propulsion);
            }
            if Reaction_wheel::get_name().eq(&devicestr) {
                ship.reaction_wheel.set_active(slot_id);
                ship.device_map.insert(slot_id, DeviceEnum::ReactionWheel);
            }
            Ok(())
        });
        methods.add_method_mut("write", |_, ship, (slot_id, addr, value):(u8, u8, u8)| {
            if ship.device_map.contains_key(&slot_id) {
                let device_enum = ship.device_map.get(&slot_id).unwrap();
                match device_enum {
                    DeviceEnum::Propulsion => {ship.propulsion.write(addr, value)},
                    DeviceEnum::ReactionWheel => {ship.reaction_wheel.write(addr, value)},
                }
            }
            Ok(())
        });
        methods.add_method("read", |_, ship, (slot_id, addr): (u8, u8)|{
            if ship.device_map.contains_key(&slot_id) {
                let device_enum = ship.device_map.get(&slot_id).unwrap();
                match device_enum {
                    DeviceEnum::Propulsion => { return Ok(ship.propulsion.read(addr)); },
                    DeviceEnum::ReactionWheel => { return Ok(ship.reaction_wheel.read(addr)); },
                }
            }
            Ok(0)
        })
    }
}