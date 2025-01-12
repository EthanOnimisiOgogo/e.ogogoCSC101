fn main() {
    let v = vec![101, 250, 330, 400];
    // Borrow v using a reference
    let v2 = &v;

    // Both v and v2 can access the vector because v2 is a reference
    println!("{:?}", v); // Original owner
    println!("{:?}", v2); // Borrowed reference
}

