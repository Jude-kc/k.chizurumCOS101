use std::io;

fn main() {
    println!("THE RESTAURANT MENU");

    println!("P - Poundo Yam / Edikaikong Soup - ₦3200");
    println!("F - Fried Rice & Chicken         - ₦3000");
    println!("A - Amala & Ewedu Soup            - ₦2500");
    println!("E - Eba & Egusi Soup              - ₦2000");
    println!("W - White Rice & Stew             - ₦2500");

    // Get food type
    println!("\nEnter food type (P, F, A, E or W):");

    let mut food_type = String::new();
    io::stdin()
        .read_line(&mut food_type)
        .expect("Failed to read input");

    let food_type = food_type.trim().to_uppercase();

    // Get quantity
    println!("Enter quantity:");

    let mut quantity_input = String::new();
    io::stdin()
        .read_line(&mut quantity_input)
        .expect("Failed to read input");

    let quantity: f64 = quantity_input
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // Determine price
    let price: f64;

    if food_type == "P" {
        price = 3200.0;
    } else if food_type == "F" {
        price = 3000.0;
    } else if food_type == "A" {
        price = 2500.0;
    } else if food_type == "E" {
        price = 2000.0;
    } else if food_type == "W" {
        price = 2500.0;
    } else {
        println!("Invalid food type!");
        return;
    }

    // Calculate total
    let total = price * quantity;

    println!("\nSubtotal: ₦{:.2}", total);

    // Apply discount
    if total > 10000.0 {
        let discount = total * 0.05;
        let final_total = total - discount;

        println!("Discount (5%): ₦{:.2}", discount);
        println!("Final Total: ₦{:.2}", final_total);
    } else {
        println!("Discount: ₦0.00");
        println!("Final Total: ₦{:.2}", total);
    }
}