//Vectors - Resizable arrays
use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5]; 
    //Re-assign values
    numbers[2] = 20;
    //Add onto vector
    numbers.push(5);
    numbers.push(6);
    //Pop off last value
    numbers.pop();
    //loop through values
    for x in numbers.iter_mut(){
        *x *= 2; 
    }

        println!("Number: {:?}",numbers);

    println!("{:?}",numbers);
    //get single value
    println!("{}",numbers[0]);
    //get Vector length
    println!("Vector Length: {}",numbers.len());
    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice); 
}
