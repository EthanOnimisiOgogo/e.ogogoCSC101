fn main() {
	let p:f64 = 520000000.0;
	let r:f64 = 10.0;
	let t:i32 = 5;

	// simple interest 

let a = p *( 1.0 +(r / 100.0)).powi(t);
println!("Compound interest for 5 years at 10 percent anually is {}", a );
let ci = a-p;
println!("Compound interest for 5 years at 10 percent anually is {}", ci, );
}