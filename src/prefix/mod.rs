use crossterm::{event::{KeyModifiers, KeyCode, KeyEvent}};
use crate::{
    types::Direction,
    key::GlobalKey,
};


#[derive(Debug)]
pub enum Prefix {
    Leader(LeaderPre),
    Buffer(BufferPre),
    Tab(TabPre),
    Find(FindPre),
    Window(WindowPre),
    Motion(MotionPre),
    Search(SearchPre),
}
impl Prefix {
    pub fn match_global_key(kv: KeyEvent) -> Option<Self> {
        if let Some(leader) = <LeaderPre as GlobalKey>::match_key(kv) {
            Some(leader)
        } else if let Some(buffer)= <BufferPre as GlobalKey>::match_key(kv) {
            Some(buffer)
        } else if let Some(tab) = <TabPre as GlobalKey>::match_key(kv) {
            Some(tab)
        } else if let Some(find) = <FindPre as GlobalKey>::match_key(kv) {
            Some(find)
        } else if let Some(window) = <WindowPre as GlobalKey>::match_key(kv) {
            Some(window)
        } else if let Some(motion) = <MotionPre as GlobalKey>::match_key(kv) {
            Some(motion)
        } else if let Some(search) = <SearchPre as GlobalKey>::match_key(kv) {
            Some(search)
        } else {
            None
        }

    }
    pub fn match_edit_key(ke: KeyEvent) -> Option<Self> {
        None
    }
    pub fn match_key(ke: KeyEvent) -> Option<Self> {
        match ke {
            LeaderPre::key() => Some(Self::leader()),
            BufferPre::key() => Some(Self::buffer()),
            TabPre::key() => Some(Self::tab()),
            FindPre::key() => Some(Self::find()),
            WindowPre::key() => Some(Self::window()),
            MotionPre::key() => Some(Self::motion()),
            SearchPre::key() => Some(Self::search()),
            _ => None
        }
    }
    pub fn leader() -> Self { Self::Leader(LeaderPre::default()) }
    pub fn buffer() -> Self { Self::Buffer(BufferPre::default()) }
    pub fn motion() -> Self { Self::Motion(MotionPre::default()) }
    pub fn search() -> Self { Self::Search(SearchPre::default()) }
    pub fn window() -> Self { Self::Window(WindowPre::default()) }
    pub fn tab() -> Self { Self::Tab(TabPre::default()) }
    pub fn find() -> Self { Self::Find(FindPre::default()) }
}

/// Function to match keypress to prefix (or lack thereof) in Edit mode
// ()
/* impl From<KeyEvent> for Option<Prefix> {
    fn from(ke: KeyEvent) -> Self {
        if let Some() = KeyEvent { modifiers: KeyModifiers::CONTROL, KeyCode::char(c) } = ke {
            match c {
                'b' => Some(Prefix::buffer()),
                't' => Some(Prefix::tab()),
                'm' => Some(Prefix::motion()),
                '/' => Some(Prefix::search()),
                'f' => Some(Prefix::find()),
                ' ' => Some(Prefix::leader()),
                'w' => Some(Prefix::window())
            }
        } else {
            None
        }

    }
} */

#[derive(Debug, Default)]
pub struct LeaderPre {

}
#[derive(Debug, Default)]
pub struct BufferPre {

}
#[derive(Debug, Default)]
pub struct TabPre {

}
#[derive(Debug, Default)]
pub struct FindPre {
    target: FindTarget,
}
#[derive(Debug, Default)]
pub struct MotionPre {
    dir: Direction,
}
#[derive(Debug, Default)]
pub struct SearchPre {
    dir: Direction,
}
#[derive(Debug, Default)]
pub struct WindowPre {
    split: bool,

}

impl GlobalKey for LeaderPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') }
    }
}
impl GlobalKey for BufferPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('e') }
    }
}
impl GlobalKey for FindPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('f') }
    }
}
impl GlobalKey for SearchPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('l') }
    }
}
impl GlobalKey for WindowPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w') }
    }
}
impl GlobalKey for MotionPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('g') }
    }
}
impl GlobalKey for TabPre {
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('\\') }
    }
}
#[derive(Debug)]
pub enum FindTarget { Files, Buffers, }
impl ToString for FindTarget {
    fn to_string(&self) -> String {
        match self {
            Self::Files => "Files".into(),
            Self::Buffers => "Buffers".into()
        }
    }
}
impl Default for FindTarget {
    fn default() -> Self {
        Self::Files
    }
}
impl ToString for Prefix {
    fn to_string(&self) -> String {
        match self {
            Prefix::Leader(LeaderPre {}) => "Leader".to_string(),
            Prefix::Buffer(BufferPre {}) => "Buffer".to_string(),
            Prefix::Tab(TabPre {}) => "Tab".to_string(),
            Prefix::Find(FindPre { target }) => format!("Find {}", target.to_string()),
            Prefix::Window(WindowPre { split }) => format!("Win (s: {})", split),
            Prefix::Motion(MotionPre { dir }) => format!("Move {}", dir.to_string()),
            Prefix::Search(SearchPre { dir }) => format!("Search {},", dir.to_string()),
        }
    }
}