extern crate lua;

//use std::{fs::File, io::Read};

pub struct Spaceautomat {
    state: lua::State
}

impl Spaceautomat {
    pub fn new() -> Spaceautomat {
        let mut state = lua::State::new();

        Spaceautomat {
            state: state,
        }
    }
    pub fn load_code(&mut self, code: &String) {
        let rc = self.state.load_string(code);
        
    }
}


//pub fn init() {
//    let mut state = lua::State::new();
//    state.open_libs();
//    let _ = state.do_string("print('hello world!')");
//}

//extern crate argparse;
//
//use argparse::{ArgumentParser, StoreTrue, List};
//use crate::spac;
//
//fn main() {
//    let mut verbose = false;
//    let mut automats: Vec<String> = vec![];
//
//    {
//        let mut ap = ArgumentParser::new();
//        ap.set_description("Space automats runtime");
//        ap.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
//        ap.refer(&mut automats).add_option(&["-a", "--automat"], List, "List of automat files");
//        ap.parse_args_or_exit();
//    }
//
//    for automat in automats {
//        let mut f = File::open(automat);
//        
//        space
//
//    }
//}
