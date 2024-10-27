fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
	let t:i32 = 3;

	// simple interest 

let a = p *( 1.0 -(r / 100.0)).powi(t);
println!("value after depreciation for 3 years at 5 percent anually is {}", a );
}