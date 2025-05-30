//! Formatted text.


use crate::value::ident::Ident;
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


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

/// Basic colours for text.
#[derive(Clone, Copy, Debug, Ser)]
pub enum TextBasicColour {

    /// `#000000`
    #[serde(rename = "black")]
    Black,

    /// `#0000AA`
    #[serde(rename = "dark_blue")]
    DarkBlue,

    /// `#00AA00`
    #[serde(rename = "dark_green")]
    DarkGreen,

    /// `#00AAAA`
    #[serde(rename = "dark_aqua")]
    DarkAqua,

    /// `#AA0000`
    #[serde(rename = "dark_red")]
    DarkRed,

    /// `#AA00AA`
    #[serde(rename = "dark_purple")]
    DarkPurple,

    /// `#FFAA00`
    #[serde(rename = "gold")]
    Orange,

    /// `#AAAAAA`
    #[serde(rename = "gray")]
    Grey,

    /// `#555555`
    #[serde(rename = "dark_gray")]
    DarkGrey,

    /// `#5555FF`
    #[serde(rename = "blue")]
    Blue,

    /// `#55FF55`
    #[serde(rename = "grene")]
    Green,

    /// `#55FFFF`
    #[serde(rename = "aqua")]
    Aqua,

    /// `#FF5555`
    #[serde(rename = "red")]
    Red,

    /// `#FF55FF`
    #[serde(rename = "light_purple")]
    Pink,

    /// `#FFFF55`
    #[serde(rename = "yellow")]
    Yellow,

    /// `#FFFFFF`
    #[serde(rename = "white")]
    White,

    /// A 24-bit RGB colour.
    #[serde(untagged, serialize_with = "rgb_to_hex")]
    Rgb {
        /// Red lane.
        r : u8,
        /// Green lane.
        g : u8,
        /// Blue lane.
        b : u8
    }

}

/// A 32-bit ARGB colour for text.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextARGBColour {
    /// Alpha lane.
    pub a : u8,
    /// Red lane.
    pub r : u8,
    /// Green lane.
    pub g : u8,
    /// Blue lane.
    pub b : u8
}
impl Ser for TextARGBColour {
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    {
        (((self.a as u32) << 24)
        | ((self.r as u32) << 16)
        | ((self.g as u32) << 8)
        | (self.b as u32)).serialize(serer)
    }
}

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

/// An event to trigger on left-clicking a text component.
#[derive(Clone, Debug, Ser)]
#[serde(tag = "action")]
pub enum TextClickEvent<'l> {

    /// Open a URL on the client.
    ///
    /// A confirmation popup will be displayed to the client.
    #[serde(rename = "open_url")]
    OpenURL {
        /// The URL to open.
        url : Cow<'l, str>
    },

    /// Runs a command.
    ///
    /// The command is run as the player, as if they typed it into chat.
    #[serde(rename = "run_command")]
    RunCommand {
        /// The command to run.
        command : Cow<'l, str>
    },

    /// Suggests a command.
    ///
    /// The suggested command will replace the current typing chat.
    ///
    /// Suggesting chat messages is possible by omitting the `/` prefix.
    #[serde(rename = "suggest_command")]
    SuggestCommand {
        /// The command to fill in chat.
        command : Cow<'l, str>
    },

    /// Changes the currently opened book page.
    #[serde(rename = "change_page")]
    SetBookPage {
        /// The page to change to.
        page : usize
    },

    /// Set the client's clipboard.
    ///
    /// This text will replace the client's clipbord.
    #[serde(rename = "copy_to_clipboard")]
    SetClipboard {
        /// The string to copy.
        value : Cow<'l, str>
    }

}

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


fn slice_is_empty<T>(slice : &[T]) -> bool { slice.is_empty() }

fn rgb_to_hex<S>(r : &u8, g : &u8, b : &u8, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
where
    S : Serer
{
    format!("#{r:0>2x}{g:0>2x}{b:0>2x}").serialize(serer)
}
