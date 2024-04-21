use crate::protocol::{event::Event, ident::Ident};

pub trait Register<I, E>
where
    I: Ident,
    E: Event,
{
    fn register(&mut self, event: E) -> I;

    fn unregister(&mut self, ident: I);
}
