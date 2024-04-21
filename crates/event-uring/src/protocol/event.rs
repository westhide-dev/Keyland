pub trait Event {
    fn process(&mut self);
}
