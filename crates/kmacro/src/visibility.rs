use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{
    parse_macro_input, Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemMod, ItemStatic,
    ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse, Visibility,
};

/// Override the visibility
pub fn make(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let vis: Visibility = parse_macro_input!(attrs);
    let mut input: Item = parse_macro_input!(input);

    #[rustfmt::skip]
    match &mut input {
        Item::Const      (item @ ItemConst      { .. }) => item.vis = vis,
        Item::Enum       (item @ ItemEnum       { .. }) => item.vis = vis,
        Item::ExternCrate(item @ ItemExternCrate{ .. }) => item.vis = vis,
        Item::Fn         (item @ ItemFn         { .. }) => item.vis = vis,
        Item::Mod        (item @ ItemMod        { .. }) => item.vis = vis,
        Item::Static     (item @ ItemStatic     { .. }) => item.vis = vis,
        Item::Struct     (item @ ItemStruct     { .. }) => item.vis = vis,
        Item::Trait      (item @ ItemTrait      { .. }) => item.vis = vis,
        Item::TraitAlias (item @ ItemTraitAlias { .. }) => item.vis = vis,
        Item::Type       (item @ ItemType       { .. }) => item.vis = vis,
        Item::Union      (item @ ItemUnion      { .. }) => item.vis = vis,
        Item::Use        (item @ ItemUse        { .. }) => item.vis = vis,

        _ => panic!("Cannot override visibility"),
    }

    input.into_token_stream().into()
}
