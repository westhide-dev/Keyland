use std::sync::atomic::{AtomicBool, Ordering};

use kcommon::nil::{Nil, NIL};

use crate::{
    ident::Ident,
    protocol::{self, event::Event, register::Register},
};

pub struct EventUring<T>
where
    T: Event,
{
    pub events: Vec<Option<T>>,
    reserve_idents: Vec<Ident>,

    pub running: AtomicBool,
}

impl<T> Register for EventUring<T>
where
    T: Event,
{
    type Event = T;
    type Ident = Ident;

    fn register(&mut self, event: T) -> Ident {
        let Self { reserve_idents, events, .. } = self;

        if let Some(mut ident) = reserve_idents.pop() {
            events[ident.idx] = Some(event);

            ident.ver += 1;

            ident
        } else {
            let idx = events.len();

            events.push(Some(event));

            Ident::new(idx, 0)
        }
    }

    fn unregister(&mut self, ident: Ident) -> Option<T> {
        let event = self.events[ident.idx].take();

        self.reserve_idents.push(ident);

        event
    }
}

impl<T> protocol::event_uring::EventUring for EventUring<T>
where
    T: Event,
{
    type Event = T;

    fn stat(&self) -> bool {
        self.is_running()
    }

    fn run(&mut self) -> Result<Nil, T::Err> {
        self.keep_running();

        while self.is_running() {
            self.dispatch()?;
        }

        Ok(NIL)
    }

    fn stop(&mut self) {
        self.stop_running();
    }
}

impl<T> EventUring<T>
where
    T: Event,
{
    #[inline]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    #[inline]
    pub fn keep_running(&self) {
        self.running.store(true, Ordering::Release);
    }

    #[inline]
    pub fn stop_running(&self) {
        self.running.store(false, Ordering::Release);
    }

    pub fn dispatch(&mut self) -> Result<Nil, T::Err> {
        for event in self.events.iter_mut().flatten() {
            event.process()?;
        }

        Ok(NIL)
    }
}
