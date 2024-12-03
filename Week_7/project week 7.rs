use std::io;

fn main() {
    println!("Select a calculation to perform:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => calculate_trapezium(),
        2 => calculate_rhombus(),
        3 => calculate_parallelogram(),
        4 => calculate_cube(),
        5 => calculate_cylinder(),
        _ => println!("Invalid choice. Please select a valid option."),
    }
}

fn calculate_trapezium() {
    let (base1, base2, height) = read_three_inputs("base1", "base2", "height");
    let area = height / 2.0 * (base1 + base2);
    println!("The area of the trapezium is: {:.2}", area);
}

fn calculate_rhombus() {
    let (diagonal1, diagonal2) = read_two_inputs("diagonal1", "diagonal2");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("The area of the rhombus is: {:.2}", area);
}

fn calculate_parallelogram() {
    let (base, altitude) = read_two_inputs("base", "altitude");
    let area = base * altitude;
    println!("The area of the parallelogram is: {:.2}", area);
}

fn calculate_cube() {
    let side = read_one_input("side");
    let area = 6.0 * side * side;
    println!("The area of the cube is: {:.2}", area);
}

fn calculate_cylinder() {
    let (radius, height) = read_two_inputs("radius", "height");
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("The volume of the cylinder is: {:.2}", volume);
}

fn read_one_input(prompt: &str) -> f64 {
    println!("Enter the {}: ", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

fn read_two_inputs(prompt1: &str, prompt2: &str) -> (f64, f64) {
    let value1 = read_one_input(prompt1);
    let value2 = read_one_input(prompt2);
    (value1, value2)
}

fn read_three_inputs(prompt1: &str, prompt2: &str, prompt3: &str) -> (f64, f64, f64) {
    let value1 = read_one_input(prompt1);
    let value2 = read_one_input(prompt2);
    let value3 = read_one_input(prompt3);
    (value1, value2, value3)
}
