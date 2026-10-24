use std::io::Write;

fn main() {
    // STEP 1: READ the details from the vectors (one vector per column)
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matrics = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = vec!["300", "100", "200", "200", "100"];

    // STEP 2: DISPLAY them on the screen
    println!("Student Name\tMatric. Number\tDepartment\tLevel");
    for i in 0..names.len() {
        println!("{}\t{}\t{}\t{}", names[i], matrics[i], departments[i], levels[i]);
    }

    // STEP 3: WRITE them to the file
    let mut file = std::fs::File::create("students.txt").expect("create failed");
    file.write_all("Student Name\tMatric. Number\tDepartment\tLevel\n".as_bytes()).expect("write failed");
    for i in 0..names.len() {
        file.write_all(names[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(matrics[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(departments[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(levels[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nData written to students.txt");
}
