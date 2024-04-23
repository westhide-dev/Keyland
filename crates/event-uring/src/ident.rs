use crate::protocol;

#[derive(Debug)]
pub struct Ident {
    pub idx: usize,
    pub ver: usize,
}

impl protocol::ident::Ident for Ident {}

impl Ident {
    pub fn new(idx: usize, ver: usize) -> Self {
        Self { idx, ver }
    }
}
