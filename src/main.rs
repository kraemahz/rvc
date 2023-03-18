use std::io;
use rvc::cache;
use clap::{Parser, command, Subcommand};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    Init,
    Add,
    Checkout,
    Commit,
    Push,
    Pull,
    Repro{ stage: Option<String> },
    Run
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    match args.action {
        Action::Init => { cache::init_rvc()? }
        _ => { unimplemented!() }
    }
    Ok(())
}
