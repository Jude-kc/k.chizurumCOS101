fn main() {
 let p = 1000.0 ;
 let r = 1.0;
 let t = 2.0;

 let a = p * (1.0 + (r /100.0)) * t;
 println!("Amount is {}", a );
 let si = a - p;
 println!("Simple Interest is {}", si );
}