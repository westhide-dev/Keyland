use std::sync::atomic::{AtomicBool, Ordering};

use crate::{ident::Ident, protocol};

pub struct EventUring<E>
where
    E: protocol::event::Event,
{
    pub stat: AtomicBool,
    pub events: Vec<E>,

    reserve_idents: Vec<Ident>,
}

impl<E> protocol::register::Register<Ident, E> for EventUring<E>
where
    E: protocol::event::Event,
{
    fn register(&mut self, event: E) -> Ident {
        let Self { reserve_idents, events, .. } = self;

        let ident = match reserve_idents.pop() {
            Some(ident) => ident,
            None => Ident::new(events.len()),
        };

        events[ident.index] = event;

        ident
    }

    fn unregister(&mut self, ident: Ident) {
        self.reserve_idents.push(ident);
        todo!("remove event")
    }
}

impl<E> protocol::event_uring::EventUring<Ident, E> for EventUring<E>
where
    E: protocol::event::Event,
{
    fn stat(&self) -> bool {
        self.stat.load(Ordering::Acquire)
    }

    fn run(&mut self) {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }
}
