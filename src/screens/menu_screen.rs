use crate::context::Context;
use crate::screens;


pub fn print_menu(context: &mut Context, from_start: bool) {
    if from_start {
        println!("=======================");
    } else {
        context.clear_screen();
    }
    println!("Task management system");
    println!("=======================");
    println!("1. Add task");
    println!("2. List tasks");
    println!("3. Exit");
    println!("=======================");
    println!("Please enter your choice: ");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => {
            screens::add_task_screen::print_add_task(context);
        }
        "2" => {
            screens::task_list_screen::print_tasks(context);
        }
        "3" => {
            println!("Bye!");
            context.exit = true;
        }
        _ => {
            print_menu(context, false);
        }
    }
}

