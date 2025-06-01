//! Formatted text.


use crate::value::ident::Ident;
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


mod component;
pub use component::*;

mod content;
pub use content::*;

mod style;
pub use style::*;

mod basic_colour;
pub use basic_colour::*;

mod argb_colour;
pub use argb_colour::*;

mod interact;
pub use interact::*;

mod click_event;
pub use click_event::*;

mod hover_event;
pub use hover_event::*;


/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component
/// https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:JSON_Text_Component
#[derive(Clone, Debug)]
pub struct Text<'l> {

    /// The components in this text.
    ///
    /// Components are displayed left-to-right.
    ///
    /// Formatting from one component may leak into the next.
    pub components : Cow<'l, [TextComponent<'l>]>

}
impl Ser for Text<'_> {
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    { self.components.serialize(serer) }
}


fn slice_is_empty<T>(slice : &[T]) -> bool { slice.is_empty() }
