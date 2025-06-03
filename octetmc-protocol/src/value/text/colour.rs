use super::*;
use crate::value::rgb::Rgb;


/// Colours for text foreground.
#[derive(Clone, Copy, Debug, Ser)]
pub enum TextColour {

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
    Rgb(Rgb)

}


fn rgb_to_hex<S>(rgb : &Rgb, serer : S) -> Result<<S as Serer>::Ok, <S as Serer>::Error>
where
    S : Serer
{
    format!("#{:0>2x}{:0>2x}{:0>2x}", rgb.r, rgb.g, rgb.b).serialize(serer)
}
