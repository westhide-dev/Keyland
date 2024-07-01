use std::ptr::NonNull;

use kcommon::nil::Nil;

use crate::{
    event_uring::EventUring,
    ident::Ident,
    protocol::{event::EventΞ, event_loop::EventLoopΞ, register::RegisterΞ},
};

#[derive(Debug)]
pub struct Handler<T>
where
    T: EventΞ,
{
    ptr: NonNull<EventUring<T>>,
}

impl<T> Handler<T>
where
    T: EventΞ,
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

    #[must_use]
    pub fn is_running(&self) -> bool {
        self.as_inner().is_running()
    }

    /// # Errors
    pub fn register_in_place(&mut self, event: T) -> Result<Ident, T> {
        self.as_inner_mut().register_in_place(event)
    }

    /// # Errors
    pub fn unregist(&mut self, ident: Ident) -> Result<T, Ident> {
        self.as_inner_mut().unregist(ident)
    }
}

impl<T> EventLoopΞ for Handler<T>
where
    T: EventΞ,
{
    type Event = T;

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

        impl EventΞ for Counter {
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

        let Ok(ident) = event_uring.register(Counter::new(0, handler)) else { unreachable!() };

        assert!(!event_uring.is_running());

        event_uring.run().ok();

        match event_uring.unregist(ident) {
            Ok(event) => assert_eq!(event.cnt, 1),
            Err(_) => unreachable!(),
        }
    }
}
