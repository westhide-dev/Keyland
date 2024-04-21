use crate::protocol::{event::Event, ident::Ident, register::Register};

pub trait EventUring<I, E>: Register<I, E>
where
    I: Ident,
    E: Event,
{
    fn stat(&self) -> bool;

    fn run(&mut self);

    fn stop(&mut self);
}
