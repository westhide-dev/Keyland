use crate::scaner::comment::Comment;

#[derive(Debug)]
#[repr(u8)]
pub enum Unit<'s> {
    // Keyword(Keyword),

    // Punctuator(Punctuator),

    // Ident(Ident<'s>),

    // /// literal
    // Lit(Lit<'s>),
    Comment(Comment<'s>),
}
