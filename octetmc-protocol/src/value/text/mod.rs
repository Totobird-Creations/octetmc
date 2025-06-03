//! Formatted text.


use crate::value::ident::Ident;
use core::fmt::{ self, Write as _ };
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

mod colour;
pub use colour::*;

mod interact;
pub use interact::*;

mod click_event;
pub use click_event::*;

mod hover_event;
pub use hover_event::*;


/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:Text_Component>
/// <br />
/// <https://minecraft.wiki/w/Java_Edition_protocol/Packets#Type:JSON_Text_Component>
#[derive(Clone, Debug)]
pub struct Text<'l, 'k> {

    /// The components in this text.
    ///
    /// Components are displayed left-to-right.
    ///
    /// Formatting from one component may leak into the next.
    pub components : Cow<'l, [TextComponent<'k>]>

}

impl Text<'_, '_> {

    /// A `Text` with no components.
    pub const EMPTY : Text<'static, 'static> = Text { components : Cow::Borrowed(&[]) };

}

impl<'l, 'k> Text<'l, 'k> {

    /// Convert the inner parts of this `Text` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `Text<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> Text<'static, 'static> {
        Text { components : Cow::Owned(match (self.components) {
            Cow::Borrowed(components) => components.iter().map(|component| component.to_static_owned()).collect(),
            Cow::Owned(components)    => components.into_iter().map(|component| component.into_static_owned()).collect()
        }) }
    }

    /// Convert the inner parts of this `Text` to their owned counterparts.
    ///  Returns the newly created `Text<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> Text<'static, 'static> {
        Text { components : Cow::Owned(self.components.iter().map(|component| component.to_static_owned()).collect()) }
    }

}

impl fmt::Display for Text<'_, '_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        for component in &*self.components {
            write!(f, "{component}")?;
        }
        Ok(())
    }
}

impl Text<'_, '_> {
    /// Writes this `Text` using the `Display` formatter, but as if
    ///  it is a `&str`. i.e., properly escaped.
    pub fn str_debug_display(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(DebugStrFormatter { f }, "{self}")
    }
}

impl Ser for Text<'_, '_> {
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    { self.components.serialize(serer) }
}


fn slice_is_empty<T>(slice : &[T]) -> bool { slice.is_empty() }



struct DebugStrFormatter<'l, 'k> {
    f : &'l mut fmt::Formatter<'k>
}

impl fmt::Write for DebugStrFormatter<'_, '_> {
    fn write_str(&mut self, s : &str) -> fmt::Result {
        write!(&mut self.f, "{}", s.escape_debug())
    }
}
