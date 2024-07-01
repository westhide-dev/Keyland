use crate::protocol::{event::EventΞ, ident::IdentΞ};

pub trait RegisterΞ {
    type Event: EventΞ;
    type Ident: IdentΞ;

    /// # Errors
    fn register(&mut self, event: Self::Event) -> Result<Self::Ident, Self::Event>;

    /// # Errors
    fn unregist(&mut self, ident: Self::Ident) -> Result<Self::Event, Self::Ident>;
}
