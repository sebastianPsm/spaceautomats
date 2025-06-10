use std::collections::HashMap;

use mlua::{UserData, UserDataMethods};

use crate::physmodel::spaceobject::Spaceobject;
use super::dev_reaction_wheel::ReactionWheel;
use super::dev_scanner::Scanner;
use super::dev_propulsion::Propulsion;
use super::dev_plasmacannon::Plasmacannon;
use super::device::Device;

enum DeviceEnum {
    Propulsion,
    ReactionWheel,
    Scanner,
    Plasmaconnon,
}

pub struct Ship {
    pub object: Spaceobject,
    pub propulsion: Propulsion,
    pub reaction_wheel: ReactionWheel,
    pub scanner: Scanner,
    pub plasmacannon: Plasmacannon,
    
    device_map: HashMap<u8, DeviceEnum>,
    name: String,
    health: u16,
    log: String,
}

impl Ship {
    pub fn new() -> Ship {
        let mut object = Spaceobject::new();
        object.set_size(50000);
        Ship {
            propulsion: Propulsion::new(),
            reaction_wheel: ReactionWheel::new(),
            scanner: Scanner::new(),
            plasmacannon: Plasmacannon::new(),
            device_map: HashMap::new(),
            object: object,
            name: "MyShip".to_string(),
            health: u16::MAX,
            log: "".to_string(),
        }
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
    pub fn apply_damage(&mut self, value: u16) {
        self.health -= value
    }
    pub fn add_log_msg(&mut self, msg: &String) {
        self.log.push_str(msg);
    }
    pub fn get_log(&self) -> String {
        self.log.clone()
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
            if ReactionWheel::get_name().eq(&devicestr) {
                ship.reaction_wheel.set_active(slot_id);
                ship.device_map.insert(slot_id, DeviceEnum::ReactionWheel);
            }
            if Scanner::get_name().eq(&devicestr) {
                ship.scanner.set_active(slot_id);
                ship.device_map.insert(slot_id, DeviceEnum::Scanner);
            }
            if Plasmacannon::get_name().eq(&devicestr) {
                ship.plasmacannon.set_active(slot_id);
                ship.device_map.insert(slot_id, DeviceEnum::Plasmaconnon);
            }
            Ok(())
        });
        methods.add_method_mut("write", |_, ship, (slot_id, addr, value):(u8, u8, u8)| {
            if ship.device_map.contains_key(&slot_id) {
                let device_enum = ship.device_map.get(&slot_id).unwrap();
                match device_enum {
                    DeviceEnum::Propulsion => {ship.propulsion.write(addr, value)},
                    DeviceEnum::ReactionWheel => {ship.reaction_wheel.write(addr, value)},
                    DeviceEnum::Scanner => {ship.scanner.write(addr, value)},
                    DeviceEnum::Plasmaconnon => {ship.plasmacannon.write(addr, value)},
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
                    DeviceEnum::Scanner => { return Ok(ship.scanner.read(addr)); },
                    DeviceEnum::Plasmaconnon => { return Ok(ship.plasmacannon.read(addr)); }
                }
            }
            Ok(0)
        });
        methods.add_method_mut("log", |_, ship, msg: String|{
            ship.add_log_msg(&msg);
            //print!("{}", msg);
            Ok(0)
        });
    }
}