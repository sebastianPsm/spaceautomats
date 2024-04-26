use mlua::{Function, Lua};

pub struct Spaceautomat {
    lua: Lua,
    initialized: bool,
}

pub enum ReturnCode {
    Ok,
    SyntaxError,
    InitFcnMissing,
    RunFcnMissing,
    InitFcnCall,
}

impl Spaceautomat {
    pub fn new() -> Spaceautomat {
        let lua = Lua::new();

        Spaceautomat {
            lua,
            initialized: false,
        }
    }
    /// Load Lua code and checks if init() and run() are available
    pub fn load_code(&mut self, code: &String) -> ReturnCode {
        /*
         * Load code string
         */
        if(self.lua.load(code).exec().is_err()) {
            return ReturnCode::SyntaxError;
        };

        /*
         * Check first init() and run()
         */
        let globals = self.lua.globals();
        if globals.contains_key("init").is_err() {
            return ReturnCode::InitFcnMissing;
        }
        if !globals.contains_key("run").is_err() {
            return ReturnCode::RunFcnMissing;
        }

        return ReturnCode::Ok;
    }
    /// Calls the init()-function from the loaded code to configure the space automat
    pub fn init(&mut self) -> ReturnCode {
        let globals = self.lua.globals();
        let init_fcn = globals.get::<_, Function>("init").unwrap();

        let _ = init_fcn.call::<_, u8>(0);
        self.initialized = true;
        return ReturnCode::Ok;
    }
    /// Returns the initialization state
    pub fn is_initialized(&self) -> bool {
        return self.initialized;
    }
}
