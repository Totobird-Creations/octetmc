use super::*;


/// The visual style of a text component.
#[derive(Clone, Debug, Ser)]
pub struct TextStyle<'l> {

    /// The colour of the text.
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour    : Option<TextBasicColour>,

    /// The font of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font      : Option<Ident<'l>>,

    /// Whether the text is bolded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold      : Option<bool>,

    /// Whether the text is italicised.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic    : Option<bool>,

    /// Whether the text is underlined.
    #[serde(rename = "underlined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline : Option<bool>,

    /// Whether the text is crossed out.
    #[serde(rename = "strikethrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike    : Option<bool>,

    /// Whether the text is obfuscated.
    ///
    /// Obfuscated texts are displayed as random characters of the same width, changing every frame.
    #[serde(rename = "obfuscated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscate : Option<bool>,

    /// The shadow colour of the text.
    #[serde(rename = "shadow_color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow    : Option<TextARGBColour>

}

impl TextStyle<'_> {

    /// A `TextStyle` with no formatting.
    pub const NONE : Self = Self {
        colour    : None,
        font      : None,
        bold      : None,
        italic    : None,
        underline : None,
        strike    : None,
        obfuscate : None,
        shadow    : None
    };

}

impl<'l> TextStyle<'l> {

    /// Convert the inner parts of this `TextStyle` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextStyle<'static>`.
    #[inline]
    pub fn into_static_owned(self) -> TextStyle<'static> {
        TextStyle {
            colour    : self.colour,
            font      : self.font.map(|font| font.into_static_owned()),
            bold      : self.bold,
            italic    : self.italic,
            underline : self.underline,
            strike    : self.strike,
            obfuscate : self.obfuscate,
            shadow    : self.shadow
        }
    }

    /// Convert the inner parts of this `TextStyle` to their owned counterparts.
    ///  Returns the newly created `TextStyle<'static>`.
    #[inline]
    pub fn to_static_owned(&self) -> TextStyle<'static> {
        TextStyle {
            colour    : self.colour,
            font      : self.font.as_ref().map(|font| font.to_static_owned()),
            bold      : self.bold,
            italic    : self.italic,
            underline : self.underline,
            strike    : self.strike,
            obfuscate : self.obfuscate,
            shadow    : self.shadow
        }
    }

}
