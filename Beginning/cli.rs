use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let name = "Jeremiah";
    let status = "100%";
    let command = args[1].clone();
    //println!("Command: {:?}", command);
    if command == "Hello"{
        println!("Hi {}, how are you?", name);
    }
    else if command == "status"{
        println!("Status is a go! Status: {}", status);
    }
    else{

        println!("Not a valid command!");
    }
}
