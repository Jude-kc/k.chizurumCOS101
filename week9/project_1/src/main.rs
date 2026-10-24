use std::io::Write;

fn main() {
    // The three categories of drinks, one array each
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // create() opens write-only and makes breweries.txt
    let mut file = std::fs::File::create("breweries.txt").expect("create failed");

    file.write_all("Nigerian Breweries Plc: Categories of Drinks\n".as_bytes()).expect("write failed");

    file.write_all("\nLager\n".as_bytes()).expect("write failed");
    for drink in lager.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    file.write_all("\nStout\n".as_bytes()).expect("write failed");
    for drink in stout.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    file.write_all("\nNon-Alcoholic\n".as_bytes()).expect("write failed");
    for drink in non_alcoholic.iter() {
        file.write_all(drink.as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("Data written to breweries.txt");
}
