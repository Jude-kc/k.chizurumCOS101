use std::io::Write;

fn main() {
    // The three separate datasets
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    // One single file for the merged data
    let mut file = std::fs::File::create("ministers.txt").expect("create failed");
    file.write_all("Commissioner\tMinistry\tZone\n".as_bytes()).expect("write failed");

    // Merge by index: row i of every dataset belongs to the same person
    for i in 0..commissioners.len() {
        file.write_all(commissioners[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(ministries[i].as_bytes()).expect("write failed");
        file.write_all("\t".as_bytes()).expect("write failed");
        file.write_all(zones[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("Merged data written to ministers.txt");
}
