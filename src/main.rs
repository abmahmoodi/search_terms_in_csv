extern crate csv;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::io::prelude::*;
use memmap::Mmap;

// Define the paths to the text file and the CSV file
const TEXT_FILE_PATH: &str = "/home/abolfazlm/Downloads/file_list2.txt";
const CSV_FILE_PATH: &str = "/home/abolfazlm/Downloads/id-folder-map.csv";

fn search_csv_file(filename: &str, term: &str, column_index: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let csv_data = std::str::from_utf8(&mmap)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_data.as_bytes());
    for result in reader.records() {
        let record = result?;
        if record.get(column_index).unwrap().trim() == term {
            println!("Found record: {:?}", record);
            return Ok(())
        }
    }
    println!("Record not found for term: {}", term);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the text file
    let text_file = File::open(Path::new(TEXT_FILE_PATH))?;
    let text_file_reader = BufReader::new(text_file);

    // Open the CSV file
    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(Path::new(CSV_FILE_PATH))?;

    const SEARCH_TERM: &str = "90727";
    // Get the index of the column to search in
    let header_to_search = "directory_id";
    let column_index = csv_reader.headers()?.iter().position(|h| h == header_to_search).ok_or("header not found")?;


    // Iterate over the lines of the text file
    for line in text_file_reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('/').collect();

        if parts.len() < 4 {
            continue; // Skip this line if it doesn't have at least 4 parts
        }

        // Extract the third item after the "/" character
        let search_term = parts.get(2).ok_or("Invalid line format")?.trim();
        search_csv_file(CSV_FILE_PATH, search_term, column_index);
    }

    Ok(())
}
