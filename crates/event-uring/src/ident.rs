use crate::protocol;

#[derive(Debug)]
pub struct Ident {
    pub idx: usize,
}

impl protocol::ident::IdentΞ for Ident {}

impl Ident {
    #[must_use]
    pub const fn new(idx: usize) -> Self {
        Self { idx }
    }
}
