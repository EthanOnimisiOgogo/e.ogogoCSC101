fn main() {
	let toshiba:f64 = 450000.0;
	let mac:f64 = 1500000.0;
	let hp:f64 = 750000.0;
	let dell:f64 = 2850000.0;
	let acer:f64 = 250000.0;
	let t:f64 = 2.0;
	let m:f64 = 1.0;
	let h:f64 = 3.0;
	let d:f64 = 3.0;
	let a:f64= 1.0;




let sum = (toshiba*t)+(mac *m)+(hp*h)+(dell*d)+(acer*a);
let number =t+m+h+d+a;
let average=sum/number;
println!("The sum of the sales of Chief Donatus and Sons Ltd is {}", sum );
println!("The average of the sales of Chief Donatus and Sons Ltd is {}", average, );
}