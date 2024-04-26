use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{event_uring::EventUring, protocol::event::Event};

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
    pub unsafe fn new(event_uring: &mut EventUring<T>) -> Self {
        Self { ptr: NonNull::new_unchecked(event_uring) }
    }
}

impl<T> Deref for Handler<T>
where
    T: Event,
{
    type Target = EventUring<T>;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> DerefMut for Handler<T>
where
    T: Event,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use kcommon::nil::{Nil, NIL};

    use super::*;
    use crate::protocol::{event_uring::EventUring as _, register::Register};

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

                self.handler.stop_running();

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
