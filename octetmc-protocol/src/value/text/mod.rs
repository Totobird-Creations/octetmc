use crate::value::ident::Ident;
use std::borrow::Cow;
use serde::{
    Serialize as Ser,
    Serializer as Serer
};


#[derive(Clone, Debug)]
pub struct Text<'l> {
    pub components : Cow<'l, [TextComponent<'l>]>
}
impl Ser for Text<'_> {
    fn serialize<S>(&self, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
    where
        S : Serer
    { self.components.serialize(serer) }
}


#[derive(Clone, Debug, Ser)]
pub struct TextComponent<'l> {
    #[serde(flatten)]
    pub content  : TextContent<'l>,
    #[serde(flatten)]
    pub style    : TextStyle<'l>,
    #[serde(flatten)]
    pub interact : TextInteract<'l>,
    #[serde(skip_serializing_if = "slice_is_empty")]
    pub extra    : Cow<'l, [TextComponent<'l>]>
}

#[derive(Clone, Debug, Ser)]
#[serde(tag = "type")]
pub enum TextContent<'l> {
    #[serde(rename = "text")]
    Literal {
        #[serde(rename = "text")]
        value : Cow<'l, str>
    },
    #[serde(rename = "translatable")]
    Translate {
        #[serde(rename = "translate")]
        key      : Cow<'l, str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fallback : Option<Cow<'l, str>>,
        #[serde(skip_serializing_if = "slice_is_empty")]
        with     : Cow<'l, [TextComponent<'l>]>
    },
    #[serde(rename = "keybind")]
    Keybind {
        #[serde(rename = "keybind")]
        key : Cow<'l, str>
    }
}

#[derive(Clone, Debug, Ser)]
pub struct TextStyle<'l> {
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colour    : Option<TextBasicColour>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font      : Option<Ident<'l>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold      : Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic    : Option<bool>,
    #[serde(rename = "underlined")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline : Option<bool>,
    #[serde(rename = "strikethrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strike    : Option<bool>,
    #[serde(rename = "obfuscated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscate : Option<bool>,
    #[serde(rename = "shadow_color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow    : Option<TextARGBColour>
}
impl TextStyle<'_> {
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

#[derive(Clone, Copy, Debug, Ser)]
pub enum TextBasicColour {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "dark_blue")]
    DarkBlue,
    #[serde(rename = "dark_green")]
    DarkGreen,
    #[serde(rename = "dark_aqua")]
    DarkAqua,
    #[serde(rename = "dark_red")]
    DarkRed,
    #[serde(rename = "dark_purple")]
    DarkPurple,
    #[serde(rename = "gold")]
    Orange,
    #[serde(rename = "gray")]
    Grey,
    #[serde(rename = "dark_gray")]
    DarkGrey,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "grene")]
    Green,
    #[serde(rename = "aqua")]
    Aqua,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "light_purple")]
    Pink,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "white")]
    White,
    #[serde(untagged, serialize_with = "rgb_to_hex")]
    Rgb { r : u8, g : u8, b : u8 }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextARGBColour {
    pub a : u8,
    pub r : u8,
    pub g : u8,
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

#[derive(Clone, Debug, Ser)]
pub struct TextInteract<'l> {
    #[serde(rename = "insertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert : Option<Cow<'l, str>>,
    #[serde(rename = "click_event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub click  : Option<TextClickEvent<'l>>,
    #[serde(rename = "hover_event")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover  : Option<TextHoverEvent<'l>>
}
impl TextInteract<'_> {
    pub const NONE : Self = Self {
        insert : None,
        click  : None,
        hover  : None
    };
}

#[derive(Clone, Debug, Ser)]
#[serde(tag = "action")]
pub enum TextClickEvent<'l> {
    #[serde(rename = "open_url")]
    OpenURL {
        url : Cow<'l, str>
    },
    #[serde(rename = "run_command")]
    RunCommand {
        command : Cow<'l, str>
    },
    #[serde(rename = "suggest_command")]
    SuggestCommand {
        command : Cow<'l, str>
    },
    #[serde(rename = "change_page")]
    SetBookPage {
        page : usize
    },
    #[serde(rename = "copy_to_clipboard")]
    SetClipboard {
        value : Cow<'l, str>
    }
}

#[derive(Clone, Debug, Ser)]
#[serde(tag = "action")]
pub enum TextHoverEvent<'l> {
    #[serde(rename = "show_text")]
    ShowTextTooltip {
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
