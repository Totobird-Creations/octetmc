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
