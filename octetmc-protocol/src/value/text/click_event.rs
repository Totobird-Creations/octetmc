use super::*;


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

impl<'l> TextClickEvent<'l> {

    /// Convert the inner parts of this `TextClickEvent` to their owned counterparts, or
    ///  take ownership if they are already owned. Returns the newly created
    ///  `TextClickEvent<'static>`.
    pub fn into_static_owned(self) -> TextClickEvent<'static> { match (self) {
        Self::OpenURL        { url }     => TextClickEvent::OpenURL        { url     : Cow::Owned(url.into_owned()) },
        Self::RunCommand     { command } => TextClickEvent::RunCommand     { command : Cow::Owned(command.into_owned()) },
        Self::SuggestCommand { command } => TextClickEvent::SuggestCommand { command : Cow::Owned(command.into_owned()) },
        Self::SetBookPage    { page }    => TextClickEvent::SetBookPage    { page    : page },
        Self::SetClipboard   { value }   => TextClickEvent::SetClipboard   { value   : Cow::Owned(value.into_owned()) }
    } }

    /// Convert the inner parts of this `TextClickEvent` to their owned counterparts.
    ///  Returns the newly created `TextClickEvent<'static>`.
    pub fn to_static_owned(&self) -> TextClickEvent<'static> { match (self) {
        Self::OpenURL        { url }     => TextClickEvent::OpenURL        { url     : Cow::Owned((&**url).to_owned()) },
        Self::RunCommand     { command } => TextClickEvent::RunCommand     { command : Cow::Owned((&**command).to_owned()) },
        Self::SuggestCommand { command } => TextClickEvent::SuggestCommand { command : Cow::Owned((&**command).to_owned()) },
        Self::SetBookPage    { page }    => TextClickEvent::SetBookPage    { page    : *page },
        Self::SetClipboard   { value }   => TextClickEvent::SetClipboard   { value   : Cow::Owned((&**value).to_owned()) }
    } }

}
