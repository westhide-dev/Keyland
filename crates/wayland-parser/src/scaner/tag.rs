use crate::scaner::att::Att;

#[derive(Debug)]
pub struct STag<'s> {
    name: &'s str,
    atts: Vec<Att<'s>>,
}

#[derive(Debug)]
pub struct ETag<'s> {
    name: &'s str,
}
