use crate::scaner::att::Att;

#[derive(Debug)]
pub struct STag<'s> {
    pub name: &'s str,
    pub atts: Vec<Att<'s>>,
}

#[derive(Debug)]
pub struct ETag<'s> {
    pub name: &'s str,
}
