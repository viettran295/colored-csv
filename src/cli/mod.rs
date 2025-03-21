use clap::Parser;

#[derive(Parser)]
#[command(about = "Colored CSV")]
pub struct Args {
    #[arg(short = 'i', long = "input")]
    pub input: String,
}