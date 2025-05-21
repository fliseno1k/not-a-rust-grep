use clap::Parser;
use std::{env::current_dir, ffi::OsString, path::PathBuf};

use crate::flags::Mode;

#[derive(Debug, Parser)]
#[command(name = "nrg", version, long_about = None)]
pub struct Arguments {
    /// Text or regex to search for
    #[arg(required = true)]
    pub pattern: String,

    /// File path or directory
    #[arg(long, default_value = default_path())]
    pub path: PathBuf,

    /// Max directory search depth
    #[arg(long, default_value_t = 3)]
    pub max_depth: usize,

    #[arg(long, default_value_t = 1)]
    pub threads: usize,

    #[arg(long, value_enum, default_value_t = Mode::Search )]
    pub mode: Mode,
}

#[inline]
fn default_path() -> OsString {
    current_dir().unwrap().into_os_string()
}
