pub fn run(){
let age: u8 = 18;
let check_id: bool = true;
let knows_person_of_age: bool = true;

//Basic if else

if age >= 21 && check_id || knows_person_of_age{
    println!("Bartender: What's your poison?");
} else if age < 21 && check_id{
    println!("Bartender: Sorry you cannot drink!");
} else {
    
    println!("Bartender: I need to see your ID!");
}


//shorthand if
let if_of_age = if age >= 21 {true} else {false};
println!("Is of Age: {}", if_of_age);


}
