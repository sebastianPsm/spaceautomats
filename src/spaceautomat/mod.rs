mod ship;
mod device;
mod dev_propulsion;
mod dev_reaction_wheel;
mod dev_scanner;
mod dev_plasmacannon;

use mlua::{Function, Lua, LuaOptions, StdLib};
use crate::spaceautomat::ship::Ship;

#[derive(Debug, Clone)]
pub enum State {
    Init,
    Run(String),
    Error(String)
}

pub struct Spaceautomat {
    id: u32,
    lua: Lua,
    step_count: u64,
    pub ship_hw: Ship,
    state: State
}

pub enum ReturnCode {
    Ok,
    SyntaxError,
    InitFcnMissing,
    RunFcnMissing,
    InitFcnCall,
    RunFcnCall,
}

impl Spaceautomat {
    pub fn new() -> Spaceautomat {
        let lua = Lua::new_with(StdLib::MATH|StdLib::STRING, LuaOptions::new()).unwrap();

        Spaceautomat {
            id: 0,
            lua,
            step_count: 0,
            ship_hw: Ship::new(),
            state: State::Init
        }
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    /// Load Lua code and checks if init() and run() are available
    pub fn load_code(&mut self, code: &str) -> ReturnCode {
        /*
         * Load code string
         */
        if self.lua.load(code).set_name("body").exec().is_err() {
            return ReturnCode::SyntaxError;
        };

        /*
         * Check first init() and run()
         */
        let globals = self.lua.globals();
        let init_fcn = globals.get::<_, Function>("init");
        let run_fcn = globals.get::<_, Function>("run");
        
        if init_fcn.is_err() {
            return ReturnCode::InitFcnMissing;
        }
        if run_fcn.is_err() {
            return ReturnCode::RunFcnMissing;
        }

        return ReturnCode::Ok;
    }
    /// Calls the init()-function from the loaded code to configure the space automat
    pub fn init(&mut self) -> ReturnCode {
        let result = self.lua.scope(|scope| {
            let ship_userdata = scope.create_userdata_ref_mut(&mut self.ship_hw).unwrap();
            let init_fcn = self.lua.globals().get::<_, Function>("init").unwrap();
            let res = init_fcn.call::<_, bool>(ship_userdata);
            return res;
        });
        if result.is_err() {
            result.unwrap();
            return ReturnCode::InitFcnCall;
        }

        self.state = State::Run(String::new());
        return ReturnCode::Ok;
    }
    /// Returns the initialization state
    pub fn is_initialized(&self) -> bool {
        match self.state {
            State::Init => { return false },
            State::Run(_) => { return true },
            State::Error(_) => { return false }
        }
    }
    /// Calls the run()-function from the loaded code once
    pub fn step(&mut self) -> ReturnCode {
        // Do a automat simulation step to update the control states
        let result = self.lua.scope(|scope| {
            let ship_userdata = scope.create_userdata_ref_mut(&mut self.ship_hw).unwrap();
            let run_fcn = self.lua.globals().get::<_, Function>("run").unwrap();
            let res = run_fcn.call::<_, bool>(ship_userdata);
            return res;
        });
        match result {
            Ok(_) => {
                self.step_count += 1;
                let mut infomsg = String::new();
                infomsg.push_str("--- Log ---\n");
                infomsg.push_str(&(self.ship_hw.get_log()));
                self.state = State::Run(infomsg.to_string());
                return ReturnCode::Ok;
            },
            Err(err) => {
                let mut errmsg = String::new();
                errmsg.push_str("--- Log ---\n");
                errmsg.push_str(&(self.ship_hw.get_log()));
                errmsg.push_str("\n\n--- Lua error ---\n");
                errmsg.push_str(&(err.to_string()));
                self.set_error(errmsg.to_string());
                return ReturnCode::RunFcnCall;
            }
        }
    }
    /// Get the number of performed simulation steps
    pub fn get_step_count(&self) -> u64 {
        return self.step_count;
    }
    pub fn get_state(&self) -> State {
        self.state.clone()
    }
    pub fn set_error(&mut self, info: String) {
        self.state = State::Error(info);
    }
}