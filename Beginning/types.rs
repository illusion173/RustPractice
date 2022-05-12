

pub fn run(){

//by default i32
 let x = 1;
//by default f64
 let y = 2.5;
//add explicit
let y: i64 = 45454545454545;

println!("Max I32: {}", std::i32::MAX);
println!("Min I64: {}", std::i64::MAX);

//boolean
let is_active = true;

println!("{:?}", (x,y,is_active));

}
