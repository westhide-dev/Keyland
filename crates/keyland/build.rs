use kcommon::nil::{Nil, NIL};
use kerror::KResult;

mod scripts;

fn main() -> KResult<Nil> {
    println!("Keyland build.rs Init");

    let profile = std::env::var("PROFILE")?;
    println!("Profile type: {profile}");

    println!("Keyland build.rs Done");
    Ok(NIL)
}
