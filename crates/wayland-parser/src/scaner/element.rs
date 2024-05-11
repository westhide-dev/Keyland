use crate::scaner::{
    tag::{ETag, STag},
    text::Text,
};

#[derive(Debug)]
pub struct Element<'s> {
    stag: STag<'s>,
    text: Text<'s>,
    etag: ETag<'s>,
}
