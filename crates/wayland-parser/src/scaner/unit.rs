use crate::scaner::{comment::Comment, element::Element};

#[derive(Debug)]
#[repr(u8)]
pub enum Unit<'s> {
    Element(Element<'s>),

    Comment(Comment<'s>),
}
