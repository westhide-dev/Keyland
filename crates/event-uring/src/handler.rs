use std::ptr::NonNull;

use kcommon::nil::Nil;

use crate::{
    event_uring::EventUring,
    ident::Ident,
    protocol::{event::Event, event_loop::EventLoop, register::Register},
};

pub struct Handler<T>
where
    T: Event,
{
    ptr: NonNull<EventUring<T>>,
}

impl<T> Handler<T>
where
    T: Event,
{
    /// # Safety
    pub unsafe fn new(event_uring: &mut EventUring<T>) -> Self {
        Self { ptr: NonNull::new_unchecked(event_uring) }
    }

    fn as_inner(&self) -> &EventUring<T> {
        unsafe { self.ptr.as_ref() }
    }

    fn as_inner_mut(&mut self) -> &mut EventUring<T> {
        unsafe { self.ptr.as_mut() }
    }

    /// # Errors
    pub fn register_in_place(&mut self, event: T) -> Result<Ident, T> {
        self.as_inner_mut().register_in_place(event)
    }

    pub fn unregister(&mut self, ident: Ident) -> Option<T> {
        self.as_inner_mut().unregister(ident)
    }
}

impl<T> EventLoop for Handler<T>
where
    T: Event,
{
    type Event = T;

    fn stat(&self) -> bool {
        self.as_inner().stat()
    }

    fn run(&mut self) -> Result<Nil, T::Err> {
        self.as_inner_mut().run()
    }

    fn stop(&mut self) {
        self.as_inner_mut().stop();
    }
}

#[cfg(test)]
mod tests {
    use kcommon::nil::{Nil, NIL};

    use super::*;

    #[test]
    fn handler() {
        struct Counter {
            cnt: usize,
            handler: Handler<Self>,
        }

        impl Counter {
            pub fn new(cnt: usize, handler: Handler<Self>) -> Self {
                Self { cnt, handler }
            }
        }

        impl Event for Counter {
            type Err = Nil;
            type Ret = Nil;

            fn process(&mut self) -> Result<Self::Ret, Self::Err> {
                self.cnt += 1;

                self.handler.stop();

                Ok(NIL)
            }
        }

        let mut event_uring = EventUring::new();

        let handler = unsafe { event_uring.handler() };

        let ident = event_uring.register(Counter::new(0, handler));

        assert!(!event_uring.is_running());

        event_uring.run().ok();

        match event_uring.unregister(ident) {
            Some(event) => assert_eq!(event.cnt, 1),
            None => unreachable!(),
        }
    }
}
