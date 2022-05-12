//Arrays - Fixed List where elements are the same data types
use std::mem;
pub fn run(){
    let mut numbers: [i32;5] = [1,2,3,4,5]; 
    //Re-assign values
    numbers[2] = 20;

    println!("{:?}",numbers);
    //get single value
    println!("{}",numbers[0]);
    //get array length
    println!("Array Length: {}",numbers.len());
    //arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}",slice);
        


}
