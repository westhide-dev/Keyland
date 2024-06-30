use crate::scaner::{
    tag::{ETag, STag},
    text::Text,
};

#[derive(Debug)]
pub struct Element<'s> {
    pub stag: STag<'s>,
    pub text: Text<'s>,
    pub etag: ETag<'s>,
}
