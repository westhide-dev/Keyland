pub trait Event {
    type Ret;
    type Err;

    fn process(&mut self) -> Result<Self::Ret, Self::Err>;
}
