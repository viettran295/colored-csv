mod utils;
mod processor;
mod cli;
mod table;
mod tui;
use std::error::Error;
use processor::CSVProcessor;
use cli::Args;
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut csv_proc = CSVProcessor::new();
    let file_content: String = csv_proc.read_csv(&args.input);
    tui::show(file_content)?;
    Ok(())
}