#![allow(dead_code)]
#![allow(unused_variables)]

use anyhow;
use arguments::Arguments;
use clap::Parser;
use flags::Mode;

mod arguments;
mod flags;

pub fn main() {
    let _ = run();
}

pub fn run() -> anyhow::Result<()> {
    let args = Arguments::parse();

    let matched = match (args.mode, args.threads) {
        (Mode::Search, 1) => search(&args),
        (Mode::Search, _) => search_parallel(&args),
        (Mode::Files, 1) => search(&args),
        (Mode::Files, _) => files_parallel(&args),
    };

    println!("{:#?}", args);
    println!("{:#?}", matched);

    Ok(())
}

fn search(args: &Arguments) -> anyhow::Result<()> {
    Ok(())
}

fn search_parallel(args: &Arguments) -> anyhow::Result<()> {
    Ok(())
}

fn files(args: &Arguments) -> anyhow::Result<()> {
    Ok(())
}

fn files_parallel(args: &Arguments) -> anyhow::Result<()> {
    Ok(())
}
