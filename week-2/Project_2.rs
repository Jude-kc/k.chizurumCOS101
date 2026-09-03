fn main() {
  let t: f64 = 450_000.00;
  let m: f64 = 1_500_000.00;
  let h: f64 = 750_000.00;
  let d: f64 = 2_850_000.00;
  let a: f64 = 250_000.00;
  //sum of sales record
   let sum = t+m+h+d+a;
   println!("Sum of amount is: {}",sum );
   //Find Average
   let total_qty: f64 = 10.0;
   let average = sum/total_qty;
   println!("Average of Sale record= {}",average );

}
