use std::env;
use backend::db::{create_task, establish_connection, query_tasks};

fn help() {
    println!("subcommands:");
    println!("      new<title>: create a new task");
}


// ANCHOR: new_task
fn new_task(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_task(&conn, &args[0]);
}
// ANCHOR_END: new_task

fn show_tasks(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }
    let conn = establish_connection();
    println!("TASK\n-----");
    for task in query_tasks(&conn) {
        println!("{}: {}", task.id, task.title);
    }
}
// ANCHOR: main
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_task(&args[2..]),
        "show" => show_tasks(&args[2..]),
        _ => help(),
    }
}
// ANCHOR_END: main
