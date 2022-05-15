use std::io::{stdin, stdout, Write};
use std::process::{Child, Command, Stdio};
use std::path::Path;
use std::env;
use std::process;

/*
fn rem_first_and_last(value: &str) -> &str{
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

*/
fn process_args(user_input: &String) -> (){ 
    if user_input == "\n"{
        return;
    }
    if user_input == "exit\n"{
        process::exit(0x0100);
    }
    let mut previous_command = None; 
    let mut commands = user_input.trim().split(" | ").peekable();
    while let Some(command) = commands.next() {
    
        let mut parts = command.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
            match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                },
                "exit" => return,
                command => {
                    let stdin = previous_command
                        .map_or(Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => { previous_command = Some(output); },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }

}




fn main() {
    let mut user_input = String::new();
    loop{ 
        let cwd = env::current_dir().unwrap();
        let path = cwd.into_os_string().into_string().unwrap();
        //let final_path = rem_first_and_last(&path);
        //let path: String = String::from(cwd.to_string_lossy());
        user_input.clear();
        print!("{}", path);
        print!("$ ");
        stdout().flush().unwrap();

        stdin().read_line(&mut user_input).unwrap();

        process_args(&user_input);

    }
}
