use crate::context::Context;
use crate::screens;


pub fn print_task_details(context: &mut Context, index: usize) {
    context.clear_screen();

    let task = &context.data.tasks[index];
    let is_complete = if task.complete { "X" } else { " " };
    println!("[{}] {} ({} hours, {})", is_complete, task.title, task.estimate, task.assignee);
    println!("=======================");
    println!("1. Mark as {}", if task.complete { "incomplete" } else { "complete" });
    println!("2. Edit");
    println!("3. Delete");
    println!("4. Back to list");
    println!("=======================");
    println!("Please enter your choice: ");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    match choice.trim() {
        "1" => {
            context.data.tasks[index].complete = !context.data.tasks[index].complete;
            print_task_details(context, index);
            context.save_system.save(&context.data);
        }
        "2" => {
            screens::edit_task_screen::print_edit_task(context, index);
        }
        "3" => {
            context.data.tasks.remove(index);
            screens::task_list_screen::print_tasks(context);
            context.save_system.save(&context.data);
        }
        "4" => {
            screens::task_list_screen::print_tasks(context);
        }
        _ => {
            print_task_details(context, index);
        }
    }
}
