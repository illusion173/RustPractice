pub fn run(){

//primitive str = immutable fixed length string somewhere in memory.
//String = growable, heap allocated data structure - use when need to modify or own string data.

let mut hello = String::from("Hello ");
    //Get length
    println!("Length {}",hello.len());
    //for Characters 
    hello.push('w');
    //For strings
    hello.push_str("orld!");
//    println!("{}",hello);
    //Capacity in bytes
    //println!("Capacity {}",hello.capacity());
    //Checking if empty returns bool
  //  println!("{}",hello.is_empty());

    //Contains sub string checks if word is in that string
    //println!("Contains 'world' {}", hello.contains("world!"));

    // replace
    //println!("Replace {}",hello.replace("World,", "There"));

    //println!("{}",hello);

    for word in hello.split_whitespace(){
        println!("{}",word);

    }


    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}",s);



}
