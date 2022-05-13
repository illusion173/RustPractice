use std::io::{self,Write};

fn factorial(number: f64)->f64{

    if number <= 1.0 {
        return 1.0;
    }

    number * factorial(number - 1.0)

}

fn main() {
    println!("Please Enter a number!");
    io::stdout().flush().unwrap();

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error Reading Input!");
   let number = user_input.trim().parse::<f64>().expect("Thats not a number!");

    println!("Factorial Answer: {}", factorial(number));

}
