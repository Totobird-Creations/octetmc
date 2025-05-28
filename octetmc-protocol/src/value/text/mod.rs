use crate::value::ident::Ident;
use std::borrow::Cow;


#[derive(Clone, Debug)]
pub struct TextComponent<'l> {
    pub content  : TextContent<'l>,
    pub style    : TextStyle<'l>,
    pub interact : TextInteract<'l>,
    pub extra    : Cow<'l, [TextComponent<'l>]>
}

#[derive(Clone, Debug)]
pub enum TextContent<'l> {
    Literal {
        value : Cow<'l, str>
    },
    Translate {
        key      : Cow<'l, str>,
        fallback : Cow<'l, str>,
        with     : Cow<'l, [TextComponent<'l>]>
    },
    Keybind {
        key : Cow<'l, str>
    }
}

#[derive(Clone, Debug)]
pub struct TextStyle<'l> {
    pub colour    : TextBasicColour,
    pub font      : Ident<'l>,
    pub bold      : bool,
    pub italic    : bool,
    pub underline : bool,
    pub strike    : bool,
    pub obfuscate : bool,
    pub shadow    : TextARGBColour
}

#[derive(Clone, Copy, Debug)]
pub enum TextBasicColour {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Orange,
    Grey,
    DarkGrey,
    Blue,
    Green,
    Aqua,
    Red,
    Pink,
    Yellow,
    White,
    Rgb { r : u8, g : u8, b : u8 }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextARGBColour {
    pub a : u8,
    pub r : u8,
    pub g : u8,
    pub b : u8
}

#[derive(Clone, Debug)]
pub struct TextInteract<'l> {
    pub insert : Option<Cow<'l, str>>,
    pub click  : TextClickEvent<'l>,
}

#[derive(Clone, Default, Debug)]
pub enum TextClickEvent<'l> {
    #[default]
    None,
    OpenURL {
        url : Cow<'l, str>
    },
    RunCommand {
        command : Cow<'l, str>
    },
    SuggestCommand {
        command : Cow<'l, str>
    },
    SetBookPage {
        page : usize
    },
    SetClipboard {
        text : Cow<'l, str>
    }
}

#[derive(Clone, Default, Debug)]
pub enum TextHoverEvent<'l> {
    #[default]
    None,
    ShowTextTooltip {
        text : Cow<'l, [TextComponent<'l>]>
    }
}
