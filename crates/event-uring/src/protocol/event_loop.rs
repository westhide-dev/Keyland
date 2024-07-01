use kcommon::nil::Nil;

use crate::protocol::event::EventΞ;

pub trait EventLoopΞ {
    type Event: EventΞ;

    /// # Errors
    fn run(&mut self) -> Result<Nil, <Self::Event as EventΞ>::Err>;

    fn stop(&mut self);
}
