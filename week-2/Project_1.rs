fn main() {
	let p: f64 = 520_000_000.0;
	let n: f64 = 5.0;
	let r: f64 = 10.0;
	// Find Amount
	let a = p * (1.0 + (r/100.0)).powf(n);
	println!("The Ammount is: {}",a );
	//find Compound Interest
	let ci = a - p;
	println!("The Compound Interest is: {}", ci);
}