use crossterm::event::{KeyModifiers, KeyCode, KeyEvent};
use crate::key::GlobalKey;

#[derive(Debug)]
pub enum Mode {
    Overview(OverviewMode),
    Command(CommandMode),
    Edit(EditMode),
    Insert(InsertMode),
}
#[derive(Debug, Default)]
pub struct OverviewMode {
    view: OverviewPane
}
#[derive(Debug, Default)]
pub struct CommandMode {
    command_buf: String,
}
#[derive(Debug, Default)]
pub struct EditMode {
}
#[derive(Debug, Default)]
pub struct InsertMode {
}
#[derive(Debug)]
pub enum OverviewPane {
    Buffers, Tabs, History
}
impl Default for OverviewPane {
    fn default() -> Self {
        Self::Buffers
    }
}
impl Mode {

    pub fn match_global_key(kv: KeyEvent) -> Option<Self> {
        if let Some(insert) = <InsertMode as GlobalKey>::match_key(kv) {
            Some(Self::Insert(insert))
        } else if let Some(overview) = <OverviewMode as GlobalKey>::match_key(kv) {
            Some(Self::Overview(overview))
        } else if let Some(edit) = <EditMode as GlobalKey>::match_key(kv) {
            Some(Self::Edit(edit))
        } else if let Some(command) = <CommandMode as GlobalKey>::match_key(kv) {
            Some(Self::Command(command))
        } else {
            None
        }
    }
    /// By default, bound to
    pub fn toggle_insert(&self) -> Self {
        match self {
            Mode::Command(_) => Mode::edit(),
            Mode::Edit(_) => Mode::insert(),
            Mode::Insert(_) => Mode::edit(),
            Mode::Overview(_) => Mode::edit(),
        }
    }
    pub fn edit() -> Self { Self::Edit(EditMode::default()) }
    pub fn insert() -> Self { Self::Insert(InsertMode::default()) }
    pub fn overview() -> Self { Self::Overview(OverviewMode::default()) }
    pub fn command() -> Self { Self::Command(CommandMode::default()) }
    pub fn toggle_insert_key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Enter }
    }
}
impl GlobalKey for InsertMode {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('v') }
    }
}
// TODO should probaly make key() return a list of valid keys?
impl GlobalKey for OverviewMode {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('z') }
    }
    fn match_key(ke: KeyEvent) -> Option<Self> {
        match ke {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('s') } |
            KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::Esc } =>
                Some(Self::default()),
            _ => None,
        }
    }
}
impl GlobalKey for EditMode {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('c') }
    }
}
impl GlobalKey for CommandMode {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('x') }
    }
}
/* impl From<KeyEvent> for Option<Mode> {
    fn from(ke: KeyEvent) -> Self {
        match ke {
            KeyEvent { Modifiers }
        }

    }
} */

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Command(_) => "COMMAND".to_string(),
            Mode::Edit(_) => "EDIT".to_string(),
            Mode::Insert(_) => "INSERT".to_string(),
            Mode::Overview(_) => "OVERVIEW".to_string(),
        }
    }
}
