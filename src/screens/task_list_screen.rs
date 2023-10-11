use crate::context::Context;
use crate::screens;


pub fn print_tasks(context: &mut Context) {
    context.clear_screen();

    println!("Tasks:");
    for (i, task) in context.data.tasks.iter().enumerate() {
        let is_complete = if task.complete { "X" } else { " " };
        println!("{}. [{}] {} ({} hours, {})", i + 1, is_complete, task.title, task.estimate, task.assignee);
    }
    println!("=======================");
    println!("0. Back to main menu");
    println!("=======================");
    println!("Please enter your choice: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    if choice.trim() == "0" {
        screens::menu_screen::print_menu(context, false);
        return;
    }
    let index = choice.trim().parse::<usize>().expect("Please enter a valid number");
    if index <= context.data.tasks.len() {
        screens::task_details_screen::print_task_details(context, index - 1);
        return;
    }

    print_tasks(context);
}