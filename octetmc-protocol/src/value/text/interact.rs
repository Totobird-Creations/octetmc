use super::*;


/// The interaction events of a text component.
///
/// Interaction events are not available in some
///  contexts, such as the server list MOTD or actionbar.
#[derive(Clone, Debug, Ser)]
pub struct TextInteract<'l> {

    /// A string to insert into chat on shift-left-clicking this text.
    #[serde(rename = "insertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert : Option<Cow<'l, str>>,

    /// An event to trigger on left-clicking this text.
    #[serde(rename = "click_event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click  : Option<TextClickEvent<'l>>,

    /// An event to trigger on hovering over this text.
    #[serde(rename = "hover_event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover  : Option<TextHoverEvent<'l>>

}

impl TextInteract<'_> {

    /// A `TextInteract` with no event triggers.
    pub const NONE : Self = Self {
        insert : None,
        click  : None,
        hover  : None
    };

}

impl<'l> TextInteract<'l> {

    /// Convert the inner parts of this `TextInteract` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextInteract<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> TextInteract<'static> {
        TextInteract {
            insert : self.insert.map(|insert| Cow::Owned(insert.into_owned())),
            click  : self.click.map(|click| click.into_static_owned()),
            hover  : self.hover.map(|hover| hover.into_static_owned()),
        }
    }

    /// Convert the inner parts of this `TextInteract` to their owned counterparts.
    ///  Returns the newly created `TextInteract<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> TextInteract<'static> {
        TextInteract {
            insert : self.insert.as_ref().map(|insert| Cow::Owned((&**insert).to_owned())),
            click  : self.click.as_ref().map(|click| click.to_static_owned()),
            hover  : self.hover.as_ref().map(|hover| hover.to_static_owned()),
        }
    }

}
