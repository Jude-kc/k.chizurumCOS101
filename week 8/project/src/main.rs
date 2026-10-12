// Project I: The APS Level Checker
// Validates a staff level for the Federal Government of Nigeria public service
// using vectors.

use std::io::{self, Write};

// Read one trimmed line from the user after printing a prompt
fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
    input.trim().to_string()
}

// Find the index of a value in a vector (case-insensitive)
fn find_index(items: &[&str], value: &str) -> Option<usize> {
    items
        .iter()
        .position(|item| item.eq_ignore_ascii_case(value))
}

fn main() {
    // The six levels, from lowest to highest (rows of the table)
    let levels: Vec<&str> = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    // Years of experience each level covers (same order as levels)
    let min_years: Vec<u32> = vec![1, 3, 5, 8, 10, 13];
    let max_years: Vec<u32> = vec![2, 5, 8, 10, 13, 100]; // SES = 13+ years

    // The four career categories (columns of the table)
    let categories: Vec<&str> = vec!["Office Admin", "Academic", "Lawyer", "Teacher"];

    // positions[c][l] = job title in category c at level l
    let positions: Vec<Vec<&str>> = vec![
        // Office Admin
        vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"],
        // Academic ("-" means no position at that level)
        vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"],
        // Lawyer
        vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"],
        // Teacher
        vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"],
    ];

    println!("=== APS Level Checker (Federal Government of Nigeria) ===\n");

    println!("Categories:");
    for c in &categories {
        println!("  - {}", c);
    }

    // 1. Category
    let category = prompt("\nEnter category: ");
    let c = match find_index(&categories, &category) {
        Some(i) => i,
        None => {
            println!("Error: unknown category.");
            return;
        }
    };

    println!("\nPositions in {}:", categories[c]);
    for p in &positions[c] {
        if *p != "-" {
            println!("  - {}", p);
        }
    }

    // 2. Position
    let position = prompt("\nEnter position: ");
    let l = match find_index(&positions[c], &position) {
        Some(i) if positions[c][i] != "-" => i,
        _ => {
            println!("Error: unknown position for {}.", categories[c]);
            return;
        }
    };

    // 3. Years of experience
    let years: u32 = match prompt("Enter years of experience: ").parse() {
        Ok(y) => y,
        Err(_) => {
            println!("Error: years of experience must be a non-negative whole number.");
            return;
        }
    };

    // 4. Validate: does the experience fit the level held by this position?
    println!("\n--- Result ---");
    println!("Position : {} ({})", positions[c][l], categories[c]);
    println!("Level    : {}", levels[l]);

    if years >= min_years[l] && years <= max_years[l] {
        println!(
            "Status   : VALID - {} year(s) of experience fits {}.",
            years, levels[l]
        );
    } else {
        let required = if l == levels.len() - 1 {
            format!("{}+ years", min_years[l])
        } else {
            format!("{} to {} years", min_years[l], max_years[l])
        };
        println!(
            "Status   : INVALID - {} requires {}, but {} was entered.",
            levels[l], required, years
        );

        // Suggest the level(s) that the experience does fit
        let matches: Vec<String> = (0..levels.len())
            .filter(|&i| years >= min_years[i] && years <= max_years[i])
            .map(|i| format!("{} ({})", levels[i], positions[c][i]))
            .collect();

        if matches.is_empty() {
            println!("Suggestion: {} year(s) fits no level.", years);
        } else {
            println!(
                "Suggestion: {} year(s) fits {}.",
                years,
                matches.join(" / ")
            );
        }
    }
}