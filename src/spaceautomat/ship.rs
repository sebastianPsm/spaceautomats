use mlua::{UserData, UserDataMethods, Scope};

use super::device::Device;
use super::dev_propulsion::Propulsion;
use super::Spaceautomat;

pub enum State {
    Init = 1,
    Run = 2
}

pub struct Ship {
    state: State,
    propulsion: Propulsion,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            state: State::Init,
            propulsion: Propulsion::new(),
        }
    }
}
impl Ship {
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }
    pub fn get_state(self) -> State {
        self.state
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
        })
    }
}

