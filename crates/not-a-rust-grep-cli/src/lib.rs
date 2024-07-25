use arguments::Arguments;
use clap::Parser;

mod arguments;

pub fn run() -> anyhow::Result<()> {
    let args = Arguments::parse();
    print!("{:?}", args);

    Ok(())
}
