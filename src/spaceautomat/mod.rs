mod ship;
mod device;
mod dev_propulsion;

use mlua::{Function, Lua};
use crate::spaceautomat::ship::Ship;
use self::ship::State;

pub struct Spaceautomat {
    lua: Lua,
    step_count: u64,
    ship: Ship,
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
            ship: Ship::new(),
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
            let globals = self.lua.globals();

            let ship_userdata = scope.create_userdata_ref_mut(&mut self.ship).unwrap();
            globals.set("ship", ship_userdata).unwrap();

            let init_fcn = globals.get::<_, Function>("init").unwrap();
            let res = init_fcn.call::<_, bool>(true);
            if res.is_err() {
                return res;
            }
            return res;
        });
        
        if result.is_err() {
            return ReturnCode::InitFcnCall;
        }

        self.ship.set_state(ship::State::Run);
        return ReturnCode::Ok;
    }
    /// Returns the initialization state
    pub fn is_initialized(&self) -> bool {
        match self.ship.get_state() {
            State::Init => { return false },
            State::Run => { return true }
        }
    }
    /// Calls the run()-function from the loaded code once
    pub fn step(&mut self) -> ReturnCode {
        let globals = self.lua.globals();
        let run_fcn = globals.get::<_, Function>("run").unwrap();

        let res = run_fcn.call::<_, bool>(true);
        if res.is_err() {
            return ReturnCode::RunFcnCall;
        }
        self.step_count += 1;

        return ReturnCode::Ok;
    }
    /// Get the number of performed simulation steps
    pub fn get_step_count(&self) -> u64 {
        return self.step_count;
    }
}