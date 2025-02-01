use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short = 'd',
        long = "database",
    )]
    pub database: PathBuf,
}
