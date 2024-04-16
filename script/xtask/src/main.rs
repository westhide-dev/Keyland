use clap::Parser;
use kcommon::nil::{Nil, NIL};
use kerror::KResult;

mod command;

/// Keyland xtask
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(long, default_value_t = String::from("xtask"))]
    name: String,

    #[arg(long)]
    verbose: bool,
}

fn main() -> KResult<Nil> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    tracing::info!("Init");

    let keyland_dir = env!("CARGO_RUSTC_CURRENT_DIR");
    let xtask_sh_file = format!("{keyland_dir}/script/sh/xtask.sh");

    command::run("bash", &[&xtask_sh_file], cli.verbose)?;

    tracing::info!("Done");

    Ok(NIL)
}
