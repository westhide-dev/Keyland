use kcommon::nil::Nil;

use crate::protocol::event::Event;

pub trait EventLoop {
    type Event: Event;

    /// # Errors
    fn run(&mut self) -> Result<Nil, <Self::Event as Event>::Err>;

    fn stop(&mut self);
}
