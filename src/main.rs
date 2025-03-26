mod utils;
mod processor;
mod cli;
mod tui;
use std::error::Error;
use processor::CSVProcessor;
use cli::Args;
use clap::Parser;
use tui::csv_data::CSVData;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut csv_proc = CSVProcessor::new();
    let file_content: CSVData = csv_proc.read_csv(&args.input);
    
    let terminal = ratatui::init();
    let app_result = tui::TableTUI::new(&file_content).run(terminal);
    ratatui::restore();
    app_result?;
    Ok(())
}