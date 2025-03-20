mod utils;
mod processor;
use std::error::Error;
use processor::CSVProcessor;

fn main() -> Result<(), Box<dyn Error>> {
    utils::init();
    let file_path = "/home/viettr/Desktop/Dev/colored-csv/test.csv";
    let mut csv_proc = CSVProcessor::new();
    let file_content: String = csv_proc.read_csv(file_path);
    println!("{}", file_content);
    Ok(())
}