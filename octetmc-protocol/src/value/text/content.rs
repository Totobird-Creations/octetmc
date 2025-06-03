use super::*;


/// The content of a text component.
#[derive(Clone, Debug, Ser)]
#[serde(tag = "type")]
pub enum TextContent<'l> {

    /// Plain text.
    #[serde(rename = "text")]
    Literal {

        /// The plain text to display directly.
        #[serde(rename = "text")]
        value : Cow<'l, str>

    },

    /// Translatable text.
    ///
    /// Translatable texts are resolved by the client based on their selected language.
    #[serde(rename = "translatable")]
    Translate {

        /// The translation identifier, corresponding to the identifiers found in loaded language files.
        #[serde(rename = "translate")]
        key      : Cow<'l, str>,

        /// If no corresponding translation can be found, this is used as the translated text.
        #[serde(skip_serializing_if = "Option::is_none")]
        fallback : Option<Cow<'l, str>>,

        /// A list of text components to be inserted into the string-interpolation slots in the translation text.
        #[serde(skip_serializing_if = "slice_is_empty")]
        with     : Cow<'l, [TextComponent<'l>]>

    },

    /// Keybind text.
    ///
    /// Keybind texts are resolved by the client based on their keybind settings.
    #[serde(rename = "keybind")]
    Keybind {

        /// The keybind identifier.
        ///
        /// https://minecraft.wiki/w/Controls#Configurable_controls
        #[serde(rename = "keybind")]
        key : Cow<'l, str>

    }

}

impl<'l> TextContent<'l> {

    /// Convert the inner parts of this `TextContent` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextContent<'static>`.
    pub fn into_static_owned(self) -> TextContent<'static> { match (self) {
        Self::Literal   { value }               => TextContent::Literal { value : Cow::Owned(value.into_owned()) },
        Self::Translate { key, fallback, with } => TextContent::Translate {
            key      : Cow::Owned(key.into_owned()),
            fallback : fallback.map(|fallback| Cow::Owned(fallback.into_owned())),
            with     : Cow::Owned(match (with) {
                Cow::Borrowed(components) => components.iter().map(|component| component.to_static_owned()).collect(),
                Cow::Owned(components)    => components.into_iter().map(|component| component.into_static_owned()).collect()
            })
        },
        Self::Keybind   { key }                 => TextContent::Keybind { key : Cow::Owned(key.into_owned()) }
    } }

    /// Convert the inner parts of this `TextContent` to their owned counterparts.
    ///  Returns the newly created `TextContent<'static>`.
    pub fn to_static_owned(&self) -> TextContent<'static> { match (self) {
        Self::Literal   { value }               => TextContent::Literal { value : Cow::Owned((**value).to_owned()) },
        Self::Translate { key, fallback, with } => TextContent::Translate {
            key      : Cow::Owned((**key).to_owned()),
            fallback : fallback.as_ref().map(|fallback| Cow::Owned((**fallback).to_owned())),
            with     : with.iter().map(|component| component.to_static_owned()).collect()
        },
        Self::Keybind   { key }                 => TextContent::Keybind { key : Cow::Owned((**key).to_owned()) }
    } }

}

impl fmt::Display for TextContent<'_> {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            TextContent::Literal   { value }             => write!(f, "{value}"),
            TextContent::Translate { key, fallback, .. } => write!(f, "{}", fallback.as_ref().unwrap_or(key)),
            TextContent::Keybind   { key }               => write!(f, "{key}"),
        }
    }
}

impl TextContent<'_> {
    /// Writes this `TextContent` using the `Display` formatter, but as if
    ///  it is a `&str`. i.e., properly escaped.
    pub fn str_debug_display(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(DebugStrFormatter { f }, "{self}")
    }
}
