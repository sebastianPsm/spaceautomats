use std::{fs, path::Path};

use spaceautomats::Simulation;

pub struct Arguments {
    pub verbose: bool,
    pub ui: bool,
    pub ego: String,
    pub dir: String,
    pub x: u32,
    pub y: u32,
    pub seed: u64
}

pub struct App {
    pub args: Arguments,
    pub sim: Simulation,   

    // Ui
    pub selected_automat: usize, 
}

impl App {
    pub fn new(args: Arguments) -> App {
        App {
            args: args,
            sim: Simulation::new(),            
            
            selected_automat: 0,
        }
    }
    pub fn is_ui(&self) -> bool {
        self.args.ui
    }
    pub fn init(&mut self) {
        /*
         * Initialize simulation
         * - load ego (if available)
         * - load dirs (if available)
         * - init simulation
         */
        
        /*
         * Load ego
         */
        let ego_path = Path::new(&self.args.ego);
        if ego_path.exists() {
            if ego_path.extension().unwrap_or_default().eq("lua") {
                self.sim.load_automat_by_file(&ego_path.to_path_buf());
            }        
        }

        /*
         * Load dir
         */
        let paths = fs::read_dir(&self.args.dir);
        if let Ok(paths) = paths {
            for entry in paths {
                if entry.is_err() { continue; }
                let entry = entry.unwrap();
                let path = entry.path();
                if path.extension().is_none() { continue; }
                if !path.extension().unwrap().eq("lua") { continue; }
                
                self.sim.load_automat_by_file(&entry.path());
            }
        }

        self.sim.init(self.args.x, self.args.y, self.args.seed);
    }
}