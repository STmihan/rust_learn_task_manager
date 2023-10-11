use crate::context::Context;
use crate::screens;


pub fn print_edit_task(context: &mut Context, index: usize) {
    context.clear_screen();
    let task = &context.data.tasks[index];
    println!("Edit task");
    println!("=======================");
    println!("1. Title: {}", task.title);
    println!("2. Assignee: {}", task.assignee);
    println!("3. Estimate: {}", task.estimate);
    println!("4. Back to task detail");
    println!("=======================");
    println!("Please enter your choice: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => {
            println!("Please enter new title: ");
            let mut title = String::new();
            std::io::stdin().read_line(&mut title).expect("Failed to read line");
            context.data.tasks[index].title = title.trim().to_string();
            print_edit_task(context, index);
        }
        "2" => {
            println!("Please enter new assignee: ");
            let mut assignee = String::new();
            std::io::stdin().read_line(&mut assignee).expect("Failed to read line");
            context.data.tasks[index].assignee = assignee.trim().to_string();
            print_edit_task(context, index);
        }
        "3" => {
            println!("Please enter new estimate: ");
            let mut estimate = String::new();
            std::io::stdin().read_line(&mut estimate).expect("Failed to read line");
            context.data.tasks[index].estimate = estimate.trim().parse::<u8>().expect("Please enter a valid number");
            print_edit_task(context, index);
        }
        "4" => {
            screens::task_details_screen::print_task_details(context, index);
        }
        _ => {
            print_edit_task(context, index);
        }
    }
    context.save_system.save(&context.data);
}
