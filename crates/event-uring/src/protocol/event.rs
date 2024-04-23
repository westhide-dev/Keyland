use kcommon::nil::Nil;

pub trait Event {
    type Err;

    fn process(&mut self) -> Result<Nil, Self::Err>;
}
