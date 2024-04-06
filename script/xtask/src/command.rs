use std::{
    io::{self, Write},
    process::Command,
};

use kerror::KResult;
use keyland::nil::{Nil, NIL};

pub fn run(program: &str, args: &[&str], verbose: bool) -> KResult<Nil> {
    let pipes = Command::new(program).args(args).output()?;

    if verbose {
        io::stdout().write_all(&pipes.stdout)?;
        io::stderr().write_all(&pipes.stderr)?;
    }

    Ok(NIL)
}
