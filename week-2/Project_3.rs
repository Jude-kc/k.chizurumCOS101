fn main() {
	let p: f64 = 210_000.00;
	let r: f64 = 5.00;
	let n: f64 = 3.00;
	//Find the Depreciation
	let d = p *(1.0 - (r/100.0)).powf(n);

println!("The Depreciation= {}",d );
}