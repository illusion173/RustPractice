pub fn run(){
    //print to console
    println!("Hello from the print.rs file!");
    //Basic Formatting
    println!("Number: {}, {}", 1, 2);

    //Positional Arguments
    println!("{0} I am a gigachad, why yes, {1} he is indeed", "Jeremiah", "Based");

    //Named Arguments
    println!("{name} likes to code in {program}",
            name = "Jeremiah",
            program = "Rust"
             );
    //Placeholder Traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //Place Holder for debug trait
    println!("{:?}",(12,true,"Hello"));

    //Basic math
    println!("10 + 10 = {}", 10+10);







}

