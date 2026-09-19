//Quadratic Roots Calculator
use std::io;

fn main() {
   println!("Quadratic Roots Calcuator");

   //Read a
   println!("Enter Value for a:");
   let mut a = String::new();
   io::stdin().read_line(&mut a).expect("Failed to read input");
   let a: f64 = a.trim().parse().expect("Please enter a valid number");

   //Read b
   println!("Enter value for b: ");
   let mut b = String::new();
   io::stdin().read_line(&mut b).expect("Failed to read input");
   let b: f64 = b.trim().parse().expect("Please enter a valid number");


   //Read c
   println!("Enter Value for c");
   let mut c = String::new();
   io::stdin().read_line(&mut c).expect("Failed to read input");
   let c:f64 = c.trim().parse().expect("Please enter a valid number");

   //Calculate Descriminant
   let d = b * b - 4.0 *a*b*c;
   println!("\n Result");
   println!("Descriminant {} ", d);

   if d > 0.0 {
    //Two Distinct Roots
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a);
    println!("Two Distinct Root");
    println!("Root 1 = {}", root1);
    println!("Root 2 = {}", root2);

   }
   else if d == 0.0{
    //Exactly one real root
    let root = -b/(2.0 * a);

    println!("Exactly one root");
    println!("Root = {}", root );
   }
   else {
    //d < 0: No real root
    println!("No real roots (Descriminant less than zero)");
   }


   


}
