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
