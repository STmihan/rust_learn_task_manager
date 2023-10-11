use crate::context::Context;

mod context;
mod task;
mod screens;
mod save_system;
mod data;

fn main() {
    let save_system = save_system::save_system::SaveSystem::new(String::from("save"));

    let mut context = Context {
        data: save_system.load(),
        save_system,
        exit: false,
    };

    loop {
        if context.exit {
            break;
        }
        screens::menu_screen::print_menu(&mut context, true);
    }
}
