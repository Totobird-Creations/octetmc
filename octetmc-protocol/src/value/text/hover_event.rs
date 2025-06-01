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
        text : Text<'l>

    }

}
