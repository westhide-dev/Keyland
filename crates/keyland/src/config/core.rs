#[derive(Debug, Default, serde::Deserialize, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
#[archive(check_bytes)]
pub struct Keyland {
    pub version: String,
}

#[cfg(test)]
mod tests {
    use kerror::KResult;

    use super::*;
    use crate::prelude::*;

    #[test]
    fn version() -> KResult<Nil> {
        let kl = toml::from_str::<Keyland>(r"version = '0.1.0'")?;

        assert_eq!(kl.version, "0.1.0");

        Ok(NIL)
    }

    #[test]
    fn rkyv() -> KResult<Nil> {
        let kl = Keyland { version: String::from("0.1.0") };

        let bytes = rkyv::to_bytes::<_, 256>(&kl)?;

        assert_eq!(bytes.as_slice(), &[48, 46, 49, 46, 48, 0, 0, 5]);

        Ok(NIL)
    }
}
