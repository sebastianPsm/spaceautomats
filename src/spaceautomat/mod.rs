mod ship;
mod device;
mod dev_propulsion;

use mlua::{Function, Lua};
use crate::spaceautomat::ship::LuaShip;


#[derive(Debug, Copy, Clone)]
pub enum State {
    Init = 1,
    Run = 2
}

pub struct Spaceautomat {
    lua: Lua,
    step_count: u64,
    lua_ship: LuaShip,
    state: State,
    pos: (u64, u64), // (x,y)
    dir: u16 // direction in deg*10 (0..3599)
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
        let lua = Lua::new();

        Spaceautomat {
            lua,
            step_count: 0,
            lua_ship: LuaShip::new(),
            state: State::Init,
            pos: (0,0),
            dir: 0,
        }
    }
    /// Load Lua code and checks if init() and run() are available
    pub fn load_code(&mut self, code: &String) -> ReturnCode {
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
            let ship_userdata = scope.create_userdata_ref_mut(&mut self.lua_ship).unwrap();
            let init_fcn = self.lua.globals().get::<_, Function>("init").unwrap();
            let res = init_fcn.call::<_, bool>(ship_userdata);
            return res;
        });
        if result.is_err() {
            result.unwrap();
            return ReturnCode::InitFcnCall;
        }

        self.state = State::Run;
        return ReturnCode::Ok;
    }
    /// Returns the initialization state
    pub fn is_initialized(&self) -> bool {
        match self.state {
            State::Init => { return false },
            State::Run => { return true }
        }
    }
    /// Calls the run()-function from the loaded code once
    pub fn step(&mut self) -> ReturnCode {
        // Do a automat simulation step to update the control states
        let result = self.lua.scope(|scope| {
            let ship_userdata = scope.create_userdata_ref_mut(&mut self.lua_ship).unwrap();
            let run_fcn = self.lua.globals().get::<_, Function>("run").unwrap();
            let res = run_fcn.call::<_, bool>(ship_userdata);
            return res;
        });
        if result.is_err() {
            result.unwrap();
            return ReturnCode::RunFcnCall;
        }

        // Do 
        //self.lua_ship.propulsion.get_fuel();

        self.step_count += 1;
        return ReturnCode::Ok;
    }
    /// Get the number of performed simulation steps
    pub fn get_step_count(&self) -> u64 {
        return self.step_count;
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