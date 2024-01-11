use std::{io::Write, collections::VecDeque, env, ffi::OsStr};

fn main() {
    let mut save_path = env::current_exe().expect("Could not get exe location.");
    save_path.pop();
    save_path.push("todo.save");
    let mut todo_list: VecDeque<String> = VecDeque::new();
    if let Err(_) = load_list(save_path.as_os_str(), &mut todo_list) {
        std::fs::File::create(save_path.as_os_str()).unwrap();
    }
    
    loop {
        println!();
        println!();
        println!();
        println!("TODO:");
        print_list(&todo_list);
        println!("--- Controls ---");
        println!("push <new_task>           // Pushes the task to the top of the list");
        println!("pop                       // Removes the topmost task");
        println!("push top <new_task>       // Adds the task to the bottom of the list");
        println!("pop top                   // Removes the bottommost task");
        println!("insert <index> <new_task> // Inserts the task at the given index");
        println!("remove <index>            // Removes the task at the given index");
        println!("move <index> <index>      // Moves a task from one index to another");
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut input = String::default();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => handle_input(input.trim().to_owned(), &mut todo_list),
            Err(_) => todo!(),
        }
        save_list(save_path.as_os_str(), &todo_list)
    }
}
fn print_list(todo_list: &VecDeque<String>) {
    for (index, task) in todo_list.iter().enumerate() {
        println!("[{index}]: {task}");
    }
}
fn handle_input(input: String, todo_list: &mut VecDeque<String>) {
    if input.to_lowercase().starts_with("push top ") {
        let parse_result: Result<String, prse::ParseError> = prse::try_parse!(input, "push top {}");
        match parse_result {
            Ok(new_task) => todo_list.push_front(new_task),
            Err(_) => println!("Not a valid command."),
        }
    } else if input.to_lowercase().starts_with("pop top") {
        todo_list.pop_front();
    } else if input.to_lowercase().starts_with("push ") {
        let parse_result: Result<String, prse::ParseError> = prse::try_parse!(input, "push {}");
        match parse_result {
            Ok(new_task) => todo_list.push_back(new_task),
            Err(_) => println!("Not a valid command."),
        }
    } else if input.to_lowercase().starts_with("pop"){
        todo_list.pop_back();
    } else if input.to_lowercase().starts_with("insert ") {
        let parse_result: Result<(usize, String), prse::ParseError> = prse::try_parse!(input, "insert {} {}");
        match parse_result {
            Ok((index, new_task)) => todo_list.insert(index, new_task),
            Err(_) => println!("Not a valid command."),
        }
    } else if input.to_lowercase().starts_with("remove ") {
        let parse_result: Result<usize, prse::ParseError> = prse::try_parse!(input, "remove {}");
        match parse_result {
            Ok(index) => {todo_list.remove(index);},
            Err(_) => println!("Not a valid command."),
        }
    } else if input.to_lowercase().starts_with("move ") {
        let parse_result: Result<(usize, usize), prse::ParseError> = prse::try_parse!(input, "move {} {}");
        match parse_result {
            Ok((from, to)) => {
                let temp = todo_list.remove(from);
                match temp {
                    Some(task) => todo_list.insert(to, task),
                    None => println!("Invalid index"),
                }
            },
            Err(_) => println!("Not a valid command."),
        }
    } else {
        println!("Not a valid command.");
    }
}
fn save_list(path: &OsStr, todo_list: &VecDeque<String>) {
    let mut str = String::default();
    for line in todo_list {
        str.push_str(&line);
        str.push('\n');
    }
    std::fs::write(path, str).unwrap();
}
fn load_list(path: &OsStr, todo_list: &mut VecDeque<String>) -> std::io::Result<()> {
    Ok(for line in std::fs::read_to_string(path)?.lines() {
            todo_list.push_back(line.to_owned());
    })
}