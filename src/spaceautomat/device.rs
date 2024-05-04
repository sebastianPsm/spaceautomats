pub trait Device {
    fn set_slot(&mut self, slot_id: u8);
    fn get_slot(&self) -> u8;

    fn write(&mut self, addr: u32, value: u8);
    fn read(&self, addr: u32) -> u8;
}