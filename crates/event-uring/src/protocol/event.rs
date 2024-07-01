pub trait EventΞ {
    type Ret;
    type Err;

    /// # Errors
    fn process(&mut self) -> Result<Self::Ret, Self::Err>;
}
