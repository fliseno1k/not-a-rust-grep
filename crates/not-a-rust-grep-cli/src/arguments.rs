use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "not-a-rust-grep", version, long_about = None)]
pub struct Arguments {
    /// Text or regex to search for
    #[arg(required = true)]
    pattern: String,

    /// File path or directory
    path: Option<PathBuf>,

    /// Max directory search depth
    #[arg(long)]
    max_depth: Option<usize>,
}
