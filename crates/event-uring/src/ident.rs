use crate::protocol;

#[derive(Debug)]
pub struct Ident {
    pub idx: usize,
}

impl protocol::ident::Ident for Ident {}

impl Ident {
    pub fn new(idx: usize) -> Self {
        Self { idx }
    }
}
