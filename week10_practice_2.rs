fn main() {

let v = vec![101, 250, 330, 400];
let v2 = &v;

println!("Original vector: {:?}", v);  // Access through original owner
println!("Borrowed reference: {:?}", v2);  // Access through reference

}


