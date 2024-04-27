use crate::protocol::{event::Event, ident::Ident};

pub trait Register {
    type Event: Event;
    type Ident: Ident;

    /// # Errors
    fn register(&mut self, event: Self::Event) -> Result<Self::Ident, Self::Event>;

    /// # Errors
    fn unregist(&mut self, ident: Self::Ident) -> Result<Self::Event, Self::Ident>;
}
