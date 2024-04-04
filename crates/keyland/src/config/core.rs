use serde::Deserialize;

#[derive(Deserialize)]
pub struct Keyland {
    version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let core = toml::from_str::<Keyland>(r#"version = '0.1.0'"#).unwrap();

        assert_eq!(core.version, "0.1.0")
    }
}
