use std::marker::PhantomData;

use crate::protocol;

pub struct EventUring<I, E>
where
    E: protocol::event::Event,
{
    pub events: Vec<E>,
    _marker_: PhantomData<I>,
}

impl<I, E> protocol::register::Register<I> for EventUring<I, E>
where
    I: protocol::ident::Ident,
    E: protocol::event::Event,
{
    fn register() -> I {
        todo!()
    }

    fn unregister(ident: &I) {
        todo!()
    }
}

impl<I, E> protocol::event_uring::EventUring<I, E> for EventUring<I, E>
where
    I: protocol::ident::Ident,
    E: protocol::event::Event,
{
    fn stat(&self) -> bool {
        todo!()
    }

    fn run(&mut self) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }
}
