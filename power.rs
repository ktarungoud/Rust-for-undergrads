use std::io;
fn read_int() -> i32 {
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Error reading number");

    num.trim()
        .parse::<i32>()
        .unwrap()
}
fn main()
{
println!("enter a num");
let mut base=read_int();
println!("enter exponent");
let mut expo=read_int();
let mut result=1;
while expo !=0
{
result*=base;
expo-=1;
}
println!("power of the num is {}",result);
}