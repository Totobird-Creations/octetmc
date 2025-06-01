use super::*;


/// A single component in a text, including content, style, and interaction events.
#[derive(Clone, Debug, Ser)]
pub struct TextComponent<'l> {

    /// The content in this text component.
    #[serde(flatten)]
    pub content  : TextContent<'l>,

    /// The formatting style of this text component.
    #[serde(flatten)]
    pub style    : TextStyle<'l>,

    /// Interaction events on this text component.
    #[serde(flatten)]
    pub interact : TextInteract<'l>,

    /// Extra text components to display to the right of this one.
    ///
    /// Formatting from an `extra` component will not leak into the components following its parent.
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub extra    : Cow<'l, [TextComponent<'l>]>

}

impl fmt::Display for TextComponent<'_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)?;
        for component in &*self.extra {
            write!(f, "{component}")?;
        }
        Ok(())
    }
}

impl TextComponent<'_> {
    /// Writes this `TextComponent` using the `Display` formatter, but as if
    ///  it is a `&str`. i.e., properly escaped.
    pub fn str_debug_display(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(DebugStrFormatter { f }, "{self}")
    }
}
