use crate::data::Data;
use crate::save_system::save_system::SaveSystem;

pub struct Context {
    pub data: Data,
    pub save_system: SaveSystem,
    pub exit: bool,
}

impl Context {
    pub fn clear_screen(&mut self) {
        print!("\x1B[2J\x1B[1;1H");
    }
}
