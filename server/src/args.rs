use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short = 'e',
        long = "env",
    )]
    pub env: PathBuf,
}
