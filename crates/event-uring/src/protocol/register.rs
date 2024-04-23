use crate::protocol::{event::Event, ident::Ident};

pub trait Register {
    type Event: Event;
    type Ident: Ident;

    fn register(&mut self, event: Self::Event) -> Self::Ident;

    fn unregister(&mut self, ident: Self::Ident) -> Option<Self::Event>;
}
