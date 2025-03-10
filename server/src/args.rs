use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(
        short = 'e',
        long = "env",
    )]
    pub env: PathBuf,

    #[arg(
        short = 'p',
        long = "port",
        default_value_t = 5149,
    )]
    pub port: u16,
}
