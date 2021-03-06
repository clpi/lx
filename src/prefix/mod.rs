use crossterm::{event::{KeyModifiers, KeyCode, KeyEvent}};
use crate::{
    key::{EditPrefixKey, GlobalPrefixKey},
    op::GlobalOp,
    types::Direction
};


#[derive(Debug, PartialEq)]
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
        if let Some(leader) = <LeaderPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Leader(leader))
        } else if let Some(buffer)= <BufferPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Buffer(buffer))
        } else if let Some(tab) = <TabPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Tab(tab))
        } else if let Some(find) = <FindPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Find(find))
        } else if let Some(window) = <WindowPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Window(window))
        } else if let Some(motion) = <MotionPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Motion(motion))
        } else if let Some(search) = <SearchPre as GlobalPrefixKey>::match_key(kv) {
            Some(Self::Search(search))
        } else {
            None
        }

    }
    pub fn match_edit_key(ke: KeyEvent) -> Option<Self> {
        None
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
/// SPECIFIC PREFIX ACTIONS
/// All of the below prefixes are global in scope, meaning they are triggered with a
/// ctrl + ? binding in any mode, and take precedence.
///

#[derive(Debug, PartialEq)]
pub enum LeaderPre {
    Cancel

}
#[derive(Debug, PartialEq)]
pub enum BufferPre {
    Cancel

}
#[derive(Debug, PartialEq)]
pub enum TabPre {
    Cancel

}
#[derive(Debug, PartialEq)]
pub enum FindPre {
    Files { dir: String },
    Buffers,
    History,
    Cancel

}
#[derive(Debug, PartialEq)]
pub enum MotionPre {
    Word(Direction, usize),
    Line(Direction, usize),
    Cancel
}
#[derive(Debug, PartialEq)]
pub enum SearchPre {
    Search(Option<Direction>, String, bool), //case insensitive?
    Replace(Option<Direction>, String, bool),
    Cancel
}
#[derive(Debug, PartialEq)]
pub enum WindowPre {
    Split{},
    Move{},
    Cancel

}

impl Default for LeaderPre { fn default() -> Self { LeaderPre::Cancel } }
impl Default for MotionPre { fn default() -> Self { MotionPre::Cancel } }
impl Default for BufferPre { fn default() -> Self { BufferPre::Cancel } }
impl Default for TabPre { fn default() -> Self { TabPre::Cancel } }
impl Default for FindPre { fn default() -> Self { FindPre::Cancel }}
impl Default for SearchPre { fn default() -> Self { SearchPre::Cancel }}
impl Default for WindowPre { fn default() -> Self { WindowPre::Cancel }}

impl GlobalPrefixKey for LeaderPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') }
    }
}
impl GlobalPrefixKey for BufferPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('e') }
    }
}
impl GlobalPrefixKey for FindPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('f') }
    }
}
impl GlobalPrefixKey for SearchPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('l') }
    }
}
impl GlobalPrefixKey for WindowPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('w') }
    }
}
impl GlobalPrefixKey for MotionPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('g') }
    }
}
impl GlobalPrefixKey for TabPre {
    type Op = GlobalOp;
    fn key() -> KeyEvent {
        KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('\\') }
    }
}
#[derive(Debug, PartialEq)]
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
            Prefix::Leader(_lp) => "Leader".to_string(),
            Prefix::Buffer(_bp) => "Buffer".to_string(),
            Prefix::Tab(_tp) => "Tab".to_string(),
            Prefix::Find(_fp) => "Find ".to_string(),
            Prefix::Window(_wp) => "Win".to_string(),
            Prefix::Motion(_wp) => "Move ".to_string(),
            Prefix::Search(_wp) => "Search".to_string(),
        }
    }
}
