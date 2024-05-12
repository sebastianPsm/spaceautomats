use mlua::{UserData, UserDataMethods};

use super::device::Device;
use super::dev_propulsion::Propulsion;

pub struct LuaShip {
    pub propulsion: Propulsion,
}

impl LuaShip {
    pub fn new() -> LuaShip {
        LuaShip {
            propulsion: Propulsion::new(),
        }
    }
}
impl UserData for LuaShip {
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