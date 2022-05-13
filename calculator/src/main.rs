use std::io::{stdin, stdout, Write};

/*
fn addition(first_number: f64, second_number: f64)->f64{
return first_number + second_number;
}

fn subtraction(first_number: f64, second_number: f64)->f64{
return first_number - second_number;
}

fn multiplication(first_number: f64, second_number: f64)->f64{
return first_number * second_number;
}

fn division(first_number: f64, second_number: f64)->f64{
return first_number/second_number;
}


fn power_multiplication(first_number: f64, second_number: f64)->f64{
    let final_value:f64;
    //let b = second_number as i32; 
    final_value = f64::powf(first_number, second_number);

return final_value;
}

fn modulus(first_number: f64, second_number: f64)->f64{
return 1.1;
}
*/
fn process_input(first_input: &String)-> () {
    //Need to put in some processing for the tokens.
    //let str_user_input = first_input.as_str();
    //let first_number = first_input.chars().nth(0).unwrap();
    //println!("{}", first_number);
    //So we need to input all the arguments into a vector str
    let args: Vec<&str> = first_input.split(" ").collect();
    //println!("{:?}", args[0]);
    if args.len() != 3 {

        return;
    }

    let first_number = args[0].parse::<f64>().unwrap();
    let operator = args[1]; 
    let second_number = args[2].parse::<f64>().unwrap();
    //println!("Operator: {:?}", operator);
    
    match operator{
        "+" => println!("Answer: {:?}", first_number + second_number),
        "-"=> println!("Answer: {:?}", first_number - second_number), 
        "/"=> println!("Answer: {:?}", first_number / second_number), 
        "*"=> println!("Answer: {:?}", first_number * second_number),
        "**"=> println!("Answer: {:?}", f64::powf(first_number, second_number)),
        "%"=> println!("Answer: {:?}", first_number % second_number),
        _=>println!("False Operation")
    };
    return;
}


fn main() {
    let mut first_input = String::new();

    println!("Welcome to the Basic calculator!");
    println!("Made by Jeremiah Webb!");
    println!("Currently only works for one operation."); 
    println!("Input an expression to Begin: "); 
    println!("Example: 1 + 1");
    println!("White Space is needed between numbers & Operators");
    loop {
    first_input.clear();
    print!(">>>> ");
    
    stdout().flush().unwrap();

    stdin().read_line(&mut first_input).unwrap();
    
    process_input(&first_input.trim().to_string());

    /*
    first.clear();
    second.clear();
    print!("Enter first number: ");    
     
    stdout().flush().unwrap();

    stdin().read_line(&mut first).unwrap();
    
    println!("Enter second number: ");
    
    stdout().flush().unwrap();
    
    stdin().read_line(&mut second).unwrap();
    
    first_num = first.trim().parse().unwrap();
    second_num = second.trim().parse().unwrap();
    */ 

    }

}
