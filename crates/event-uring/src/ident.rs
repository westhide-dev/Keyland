use crate::protocol;

#[derive(Debug, Clone)]
pub struct Ident {
    pub index: usize,
}

impl protocol::ident::Ident for Ident {}

impl Ident {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
