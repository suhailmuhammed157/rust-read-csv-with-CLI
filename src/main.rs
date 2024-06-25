extern crate csv;

use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv_from_path<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    let matches = App::new("CSV Reader")
        .version("1.0")
        .author("Your Name")
        .about("Reads CSV files")
        .arg(
            Arg::with_name("FILE")
                .help("Sets the input CSV file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    if let Err(e) = read_csv_from_path(filename) {
        eprintln!("Error while reading file: {:?}", e);
    }
}
