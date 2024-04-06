mod command;

use clap::Parser;

/// Keyland xtask
#[derive(Debug, Parser)]
#[command(version)]
struct Cli {
    #[arg(long, default_value_t = String::from("xtask"))]
    name: String,

    #[arg(long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("Init: {}", cli.name);

    let keyland_dir = env!("CARGO_RUSTC_CURRENT_DIR");
    let xtask_sh_file = format!("{keyland_dir}/script/sh/xtask.sh");

    command::run("bash", &[&xtask_sh_file], cli.verbose).unwrap();

    println!("Done: {}", cli.name);
}
