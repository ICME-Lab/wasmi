use clap::Parser;
use wasmi_cli::{args::Args, run};

fn main() -> anyhow::Result<()> {
    run(Args::parse())
}
