#[derive(Debug)]
pub struct Comment<'s> {
    pub content: &'s str,
}
