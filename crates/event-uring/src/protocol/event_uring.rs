use kcommon::nil::Nil;

use crate::protocol::event::Event;

pub trait EventUring {
    type Event: Event;

    fn stat(&self) -> bool;

    fn run(&mut self) -> Result<Nil, <Self::Event as Event>::Err>;

    fn stop(&mut self);
}
