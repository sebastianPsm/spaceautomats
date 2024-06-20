pub trait Device {
    fn get_name() -> String;

    fn set_active(&mut self, slot_id: u8);
    fn write(&mut self, addr: u8, value: u8);
    fn read(&self, addr: u8) -> u8;
}