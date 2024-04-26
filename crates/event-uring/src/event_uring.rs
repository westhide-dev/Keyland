use std::sync::atomic::{AtomicBool, Ordering};

use kcommon::nil::{Nil, NIL};

use crate::{
    handler::Handler,
    ident::Ident,
    protocol::{event::Event, event_loop::EventLoop, register::Register},
};

#[derive(Debug, Default)]
pub struct EventUring<T>
where
    T: Event,
{
    events: Vec<Option<T>>,
    // unregisted reserve idents
    idents: Vec<Ident>,

    running: AtomicBool,
}

impl<T> Register for EventUring<T>
where
    T: Event,
{
    type Event = T;
    type Ident = Ident;

    fn register(&mut self, event: T) -> Ident {
        if let Some(ident) = self.idents.pop() {
            self.fill_event(ident.idx, event);

            ident
        } else {
            self.push_event(event);

            Ident::new(self.size() - 1)
        }
    }

    fn unregister(&mut self, ident @ Ident { idx, .. }: Ident) -> Option<T> {
        self.idents.push(ident);

        self.take_event(idx)
    }
}

impl<T> EventLoop for EventUring<T>
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
    // pub unsafe fn get_mut_unchecked<'a>(&mut self) -> &'a mut Self {
    //     std::mem::transmute(self as *mut Self)
    // }

    pub const fn new() -> Self {
        Self { events: Vec::new(), idents: Vec::new(), running: AtomicBool::new(false) }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.events.len()
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    #[inline]
    fn keep_running(&self) {
        self.running.store(true, Ordering::Release);
    }

    #[inline]
    fn stop_running(&self) {
        self.running.store(false, Ordering::Release);
    }

    fn take_event(&mut self, idx: usize) -> Option<T> {
        self.events[idx].take()
    }

    fn fill_event(&mut self, idx: usize, event: T) {
        self.events[idx] = Some(event);
    }

    fn push_event(&mut self, event: T) {
        self.events.push(Some(event));
    }

    fn dispatch(&mut self) -> Result<Nil, T::Err> {
        for event in self.events.iter_mut().flatten() {
            event.process()?;
        }

        Ok(NIL)
    }

    pub unsafe fn handler(&mut self) -> Handler<T> {
        Handler::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_uring() {
        struct Counter {
            cnt: usize,
        }

        impl Counter {
            pub fn new(cnt: usize) -> Self {
                Self { cnt }
            }
        }

        impl Event for Counter {
            type Err = Nil;
            type Ret = Nil;

            fn process(&mut self) -> Result<Self::Ret, Self::Err> {
                self.cnt += 1;
                Ok(NIL)
            }
        }

        let mut event_uring = EventUring::new();

        event_uring.register(Counter::new(0));

        assert!(!event_uring.is_running());
    }
}
