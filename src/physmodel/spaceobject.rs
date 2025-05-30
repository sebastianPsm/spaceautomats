#[derive(Debug, Copy, Clone)]
pub struct Spaceobject {
    size: u32, // radius
    pos: (u32, u32), // (x,y)
    speed: (f64, f64),
    dir: u16, // direction in deg*10 (0..3599)
    angular_velocity: f64,
}
impl Spaceobject {
    pub fn new() -> Spaceobject {
        Spaceobject {
            size: 0,
            pos: (0, 0),
            speed: (0.0, 0.0),
            dir: 0,
            angular_velocity: 0.0
        }
    }
    /// Get size
    pub fn get_size(&self) -> u32 {
        self.size
    }
    /// Set size
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
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

    pub fn check_collision(&self, other: &Spaceobject) -> bool {
        let ego_pos = self.get_pos();
        let ego_pos = (ego_pos.0 as i64, ego_pos.1 as i64);
        let other_pos = other.get_pos();
        let other_pos = (other_pos.0 as i64, other_pos.1 as i64);
        let ego_size = self.get_size() as i64;
        let other_size = other.get_size() as i64;
        let d_pos = ((ego_pos.0-other_pos.0).abs(),(ego_pos.1-other_pos.1).abs());

        let distance = (d_pos.0.pow(2) + d_pos.1.pow(2)).isqrt();
        distance < ego_size + other_size
    }
}