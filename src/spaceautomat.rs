use mlua::{Function, Lua, UserData, UserDataMethods};

pub struct Spaceautomat {
    lua: Lua,
    initialized: bool,
    step_count: u64,
}
struct Ship {
    a: i64,
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
            initialized: false,
            step_count: 0
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

        /*
         * Add ship api
         */
        impl UserData for Ship {
            fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
                methods.add_method_mut("slot", |_, ship, slot_idx: i64| {
                    // Todo: Add pheriphery to slot logic here
                    ship.a = slot_idx;

                    Ok(())
                });
            }
        }
        let _ = globals.set("ship", Ship {
            a: 0,
        });

        return ReturnCode::Ok;
    }
    /// Calls the init()-function from the loaded code to configure the space automat
    pub fn init(&mut self) -> ReturnCode {
        let globals = self.lua.globals();
        let init_fcn = globals.get::<_, Function>("init").unwrap();

        if init_fcn.call::<_, bool>(true).is_err() {
            return ReturnCode::InitFcnCall;
        }
        self.initialized = true;
        return ReturnCode::Ok;
    }
    /// Returns the initialization state
    pub fn is_initialized(&self) -> bool {
        return self.initialized;
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
    pub fn get_step_count(&self) -> u64 {
        return self.step_count;
    }
}
