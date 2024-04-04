use serde::Deserialize;

#[derive(Deserialize)]
pub struct Keyland {
    version: String,
}

#[cfg(test)]
mod tests {
    use kerror::KResult;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn run() -> KResult<Nil> {
        let core = toml::from_str::<Keyland>(r#"version = '0.1.0'"#)?;

        assert_eq!(core.version, "0.1.0");

        Ok(NIL)
    }
}
