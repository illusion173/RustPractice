//Enums are types which have definite values.
enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    //Perform action depending on info

    match m {
        Movement::Up => println!("I am Moving UP!"),
        Movement::Down => println!("I am Moving Down!"),
        Movement::Left => println!("I am Moving Left!"),
        Movement::Right => println!("I am Moving Right!")
    }
}

pub fn run(){
        let avatar1 = Movement::Left; 
        let avatar2 = Movement::Up;
        let avatar3 = Movement::Down;
        let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
