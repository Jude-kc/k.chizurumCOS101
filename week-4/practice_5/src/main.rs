//Rust program to read thr height of a height
//and then print if person is, dwarf or average height person
use std::io;
fn main() {
    let mut input = String::new();
     println!("\n Enter your Height (in centimetres):");
     io::stdin().read_line(&mut input).expect("Not a valid string");
     let height:f32 = input.trim().parse().expect("Not a valid number");

     if height >= 150.0 && height <= 170.0
     {
        println!("Ypu are of average height person");
     }
     else if height > 170.0 && height <= 195.0
     {
        println!("You are tall");
     }
     else if height < 150.0 && height > 100.0
     {
        println!("You are a dwarf");
     }
     else
     {
        println!("Abnormal height");
     }
}
    