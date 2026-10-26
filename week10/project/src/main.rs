//declare a structure for a laptop
struct Laptop {
   brand:String,
   price:u32
}

//logic to calculate the cost of buying some laptops of one brand
impl Laptop {
   fn cost(&self, quantity:u32)->u32 {
      //use the . operator to fetch the value of a field via the self keyword
      self.price * quantity
   }
}

fn main() {
   //initialize a structure for each brand
   let hp = Laptop {
      brand:String::from("HP"),
      price:650000
   };
   let ibm = Laptop {
      brand:String::from("IBM"),
      price:755000
   };
   let toshiba = Laptop {
      brand:String::from("Toshiba"),
      price:550000
   };
   let dell = Laptop {
      brand:String::from("Dell"),
      price:850000
   };

   //cost of 3 laptops from each brand
   println!("{}: 3 laptops at N{} each = N{}", hp.brand, hp.price, hp.cost(3));
   println!("{}: 3 laptops at N{} each = N{}", ibm.brand, ibm.price, ibm.cost(3));
   println!("{}: 3 laptops at N{} each = N{}", toshiba.brand, toshiba.price, toshiba.cost(3));
   println!("{}: 3 laptops at N{} each = N{}", dell.brand, dell.price, dell.cost(3));

   //one total across the four brands
   let total = hp.cost(3) + ibm.cost(3) + toshiba.cost(3) + dell.cost(3);
   println!("\nTotal cost of 3 laptops from each brand = N{}", total);
}