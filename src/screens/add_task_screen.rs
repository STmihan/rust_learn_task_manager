use crate::context::Context;
use crate::task::Task;
use crate::screens;

pub fn print_add_task(context: &mut Context) {
    context.clear_screen();
    println!("Add task");
    println!("=======================");
    println!("Please enter the title: ");
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).expect("Failed to read line");
    println!("Please enter the assignee: ");
    let mut assignee = String::new();
    std::io::stdin().read_line(&mut assignee).expect("Failed to read line");
    println!("Please enter the estimate (in hours): ");
    let mut estimate = String::new();
    std::io::stdin().read_line(&mut estimate).expect("Failed to read line");
    let estimate: u8 = estimate.trim().parse().expect("Please enter a number");
    context.data.tasks.push(Task {
        title: title.trim().to_string(),
        assignee: assignee.trim().to_string(),
        estimate,
        complete: false,
    });
    println!("Task added!");
    context.save_system.save(&context.data);
    screens::menu_screen::print_menu(context, false);
}