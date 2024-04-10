extern crate lua;

use std::{fs::File, io::Read};

pub struct Spaceautomats {
    cnt: u64,
    automats: Vec<String>
}

impl Spaceautomats {
    pub fn new() -> Spaceautomats {
        Spaceautomats {
            cnt : 0,
            automats: Vec::new()
        }
    }
    pub fn load_automat(&mut self, automat: String) {
        self.automats.push(automat);
        self.cnt = self.cnt + 1;
    }
    pub fn count(&self) -> u64 {
        return self.cnt;
    }
    pub fn term(&mut self) {
        while !self.automats.is_empty() {
            self.automats.pop();
            self.cnt = self.cnt - 1;
        }
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
