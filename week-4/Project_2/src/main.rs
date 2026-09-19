
use std::io;

fn main() {
    println!(" Incentive Calculator");

    //Input: Experienced Status 
    println!("Is Employee Exprienced (yes/no):");
    let mut exp_input = String::new();
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");

    let is_exp = exp_input.trim().to_lowercase() == "yes";

    if !is_exp {
        //Not Experience
        println!("Annal Incentive: 100,000 naira ");

    }
    else {
        //Experienced
        println!("Enter employee's age:");
        let mut age_input = String::new();
        io::stdin().read_line(&mut age_input).expect("Failed to read input");
        let age:u32 = age_input.trim().parse().expect("Please enter a number");

        let incentive;

        if age >= 40 {
            incentive = 1_560_000;

        } else if  age >= 30 {
            incentive = 1_480_000;
        }
        else {
            incentive = 1_300_000;
        }
        println!("Annual Incentive: {}naira", incentive);

    }

}
