use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    /// Text or regex to search for
    #[arg(required = true)]
    pattern: String,

    /// File path or directory
    path: Option<PathBuf>,
}
