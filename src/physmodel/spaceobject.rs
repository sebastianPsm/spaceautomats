pub struct Spaceobject {
    pos: (u32, u32), // (x,y)
    speed: (f64, f64),
    dir: u16, // direction in deg*10 (0..3599)
    angular_velocity: f64,
}
impl Spaceobject {
    pub fn new() -> Spaceobject {
        Spaceobject {
            pos: (0, 0),
            speed: (0.0, 0.0),
            dir: 0,
            angular_velocity: 0.0
        }
    }
    /// Get position
    pub fn get_pos(&self) -> (u32, u32) {
        self.pos
    }
    /// Set position
    pub fn set_pos(&mut self, pos: (u32, u32)) {
        self.pos = pos;
    }
    /// Get speed
    pub fn get_speed(&self) -> (f64, f64) {
        self.speed
    }
    /// Set speed
    pub fn set_speed(&mut self, value: (f64, f64)) {
        self.speed = value;
    }
    /// Get direction
    pub fn get_dir(&self) -> u16 {
        self.dir
    }
    /// Set direction
    pub fn set_dir(&mut self, dir: u16) {
        self.dir = dir%3600;
    }
    /// Get direction [rad]
    pub fn get_dir_rad(&self) -> f64 {
        (self.dir as f64) / 10.0 / 180.0 * std::f64::consts::PI
    }
    /// Set direction [rad]
    pub fn set_dir_rad(&mut self, dir_rad: f64) {
        let mut dir = dir_rad / std::f64::consts::PI * 180.0 * 10.0;
        while dir < 0.0 {
            dir += 3600.0
        }

        self.set_dir(dir as u16)
    }
    /// Get angular velocity [rad/step]
    pub fn get_angular_velocity_rad(&self) -> f64 {
        self.angular_velocity
    }
    /// Set angular velocity [rad/step]
    pub fn set_angular_velocity_rad(&mut self, ang_vel_rad: f64) {
        self.angular_velocity = ang_vel_rad;
    }
}