fn main() {
    // Create a vector `v` that owns the object in the heap
    let v = vec![20, 40, 60, 80];

    // Transfer ownership of `v` to `v2` and then to the `display` function
    let v2_return = display(v);

    // Ownership of `v2_return` is back in `main` after being returned by `display`
    println!("In main {:?}", v2_return);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    // Print the vector and return it
    println!("Inside display {:?}", v);
    v // Returning the vector
}

