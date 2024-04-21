use crate::protocol::ident::Ident;

pub trait Register<I>
where
    I: Ident,
{
    fn register() -> I;

    fn unregister(ident: &I);
}
