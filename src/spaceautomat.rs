use lua::ThreadStatus;

extern crate lua;

//use std::{fs::File, io::Read};

pub struct Spaceautomat {
    state: lua::State
}

pub enum ReturnCode {
    Ok,
    SyntaxError,
    InitFcnMissing,
    RunFcnMissing,
}

impl Spaceautomat {
    pub fn new() -> Spaceautomat {
        let state = lua::State::new();

        Spaceautomat {
            state: state,
        }
    }
    /// Load Lua code and checks if init() and run() are available
    pub fn load_code(&mut self, code: &String) -> ReturnCode {
        /*
         * Load code string
         */
        let rc = self.state.load_string(code);
        if matches!(rc, ThreadStatus::SyntaxError) {
            return ReturnCode::SyntaxError;
        }

        /*
         * Check fir init() and run()
         */
        let _ = self.state.pcall(0, 0, 0);

        let lua_init_fcn = self.state.get_global("init");
        if lua_init_fcn == lua::Type::Nil {
            return ReturnCode::InitFcnMissing;
        }
        let info = self.state.get_info(">Snu").unwrap();
        println!("info():{} nparams: {}, nups: {}", info.linedefined, info.nparams, info.nups);
        self.state.pop(1);

        let lua_run_fcn = self.state.get_global("run");
        if lua_run_fcn == lua::Type::Nil {
            return ReturnCode::RunFcnMissing;
        }
        let info = self.state.get_info(">Snu").unwrap();
        println!("info():{} nparams: {}, nups: {}", info.linedefined, info.nparams, info.nups);
        self.state.pop(1);

        return ReturnCode::Ok;
    }
    /// Calls the init()-function from the loaded code to configure the space automat
    pub fn init(&mut self) {
        
    }
}
