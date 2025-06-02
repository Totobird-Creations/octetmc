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

impl<'l> TextComponent<'l> {

    /// Convert the inner parts of this `TextComponent` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextComponent<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> TextComponent<'static> {
        TextComponent {
            content  : self.content.into_static_owned(),
            style    : self.style.into_static_owned(),
            interact : self.interact.into_static_owned(),
            extra    : Cow::Owned(match (self.extra) {
                Cow::Borrowed(extra) => extra.iter().map(|component| component.to_static_owned()).collect(),
                Cow::Owned(extra)    => extra.into_iter().map(|component| component.into_static_owned()).collect()
            })
        }
    }

    /// Convert the inner parts of this `TextComponent` to their owned counterparts.
    ///  Returns the newly created `TextComponent<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> TextComponent<'static> {
        TextComponent {
            content  : self.content.to_static_owned(),
            style    : self.style.to_static_owned(),
            interact : self.interact.to_static_owned(),
            extra    : Cow::Owned(self.extra.iter().map(|component| component.to_static_owned()).collect())
        }
    }

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
