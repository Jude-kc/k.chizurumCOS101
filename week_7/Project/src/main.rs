use std::io;

// 1. Trapezium Area
fn trapezium_area() {
    let height = get_input("Enter height: ");
    let base1 = get_input("Enter base 1: ");
    let base2 = get_input("Enter base 2: ");
    
    let area = height / 2.0 * (base1 + base2);
    println!("Area = {}\n", area);
}

// 2. Rhombus Area
fn rhombus_area() {
    let d1 = get_input("Enter diagonal 1: ");
    let d2 = get_input("Enter diagonal 2: ");
    
    let area = 0.5 * d1 * d2;
    println!("Area = {}\n", area);
}

// 3. Parallelogram Area
fn parallelogram_area() {
    let base = get_input("Enter base: ");
    let altitude = get_input("Enter altitude: ");
    
    let area = base * altitude;
    println!("Area = {}\n", area);
}

// 4. Cube Surface Area
fn cube_area() {
    let side = get_input("Enter side: ");
    
    let area = 6.0 * side * side;
    println!("Surface Area = {}\n", area);
}

// 5. Cylinder Volume
fn cylinder_volume() {
    let radius = get_input("Enter radius: ");
    let height = get_input("Enter height: ");
    
    let volume = std::f64::consts::PI * radius * radius * height;
    println!("Volume = {}\n", volume);
}

// Simple helper to get number input
fn get_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("Choose a shape:");
    println!("1. Trapezium (Area)[cite: 1]");
    println!("2. Rhombus (Area)[cite: 1]");
    println!("3. Parallelogram (Area)[cite: 1]");
    println!("4. Cube (Surface Area)[cite: 1]");
    println!("5. Cylinder (Volume)[cite: 1]");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => trapezium_area(),
        "2" => rhombus_area(),
        "3" => parallelogram_area(),
        "4" => cube_area(),
        "5" => cylinder_volume(),
        _ => println!("Invalid choice!"),
    }
}