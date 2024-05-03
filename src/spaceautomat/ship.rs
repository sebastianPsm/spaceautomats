use mlua::{UserData, UserDataMethods};

pub struct Ship {
    devicetbl: Vec<(u8,String)>
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            devicetbl: vec![],
        }
    }
}

impl UserData for Ship {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method_mut("slot", |_, ship, (slot, device): (u8, String)| {
            ship.devicetbl.push((slot, device));
            Ok(())
        });
    }
}