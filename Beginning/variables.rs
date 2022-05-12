

// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block scopes language


pub fn run(){

    let name = "Jeremiah";
    let mut age = 20;
   println!("My name is {}, and my age is {}", name, age);
    age = 21;
    println!("My name is {}, and my age is {}", name, age);
    
    //Define Constants
    const ID: i32 = 001;

    println!("ID: {}", ID);
    //Making multiple variables 
    let(my_name, my_age) = ("Jeremiah", 20);

    println!("My name is {}, and my age is {}",my_name, my_age);


}
