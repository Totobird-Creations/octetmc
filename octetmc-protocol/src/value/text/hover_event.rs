use super::*;


/// An event to trigger on hovering over a text component.
#[derive(Clone, Debug, Ser)]
#[serde(tag = "action")]
pub enum TextHoverEvent<'l> {

    /// Show a text component in a tooltip.
    #[serde(rename = "show_text")]
    ShowTextTooltip {

        /// The text to show.
        #[serde(rename = "value")]
        text : Text<'l, 'l>

    }

}

impl<'l> TextHoverEvent<'l> {

    /// Convert the inner parts of this `TextHoverEvent` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextHoverEvent<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> TextHoverEvent<'static> { match (self) {
        Self::ShowTextTooltip { text } => TextHoverEvent::ShowTextTooltip { text : text.into_static_owned() }
    } }

    /// Convert the inner parts of this `TextHoverEvent` to their owned counterparts.
    ///  Returns the newly created `TextHoverEvent<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> TextHoverEvent<'static> { match (self) {
        Self::ShowTextTooltip { text } => TextHoverEvent::ShowTextTooltip { text : text.to_static_owned() }
    } }

}
