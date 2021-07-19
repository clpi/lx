use crate::error::{LxConfigError, LxError};
use serde::{Serializer, Deserializer, Serialize, Deserialize};
use std::convert::TryFrom;
use crossterm::{
    event::{Event, KeyEvent, KeyModifiers, KeyCode},
};

#[derive(Debug, PartialEq)]
pub struct KeyEv(KeyEvent);

#[derive(Debug)]
pub enum KeyMap {
    Char(char),
    Ctrl(char),
    Alt(char),
    AltShift(char),
    CtrlShift(char),
    CtrlAlt(char),
}

impl TryFrom<String> for KeyMap {
    type Error = LxConfigError;

    fn try_from(map: String) -> Result<Self, Self::Error> {
        let map_lc = map.to_lowercase();
        if map.len() == 1 {
            let ch = map.chars().nth(0).unwrap();
            return Ok( KeyMap::Char(ch) );
        } else if map.len() == 3 {
            if map_lc.starts_with("c-") {
                let ch = map.chars().nth(2).unwrap();
                return Ok( KeyMap::Ctrl(ch) )
            } else if map_lc.starts_with("a-") {
                let ch = map.chars().nth(2).unwrap();
                return Ok( KeyMap::Alt(ch) )
            }
        } else if map.len() == 5 {
            if map_lc.starts_with("c-s-") | map_lc.starts_with("s-c-") {
                let ch = map.chars().nth(4).unwrap();
                return Ok( KeyMap::CtrlShift(ch))
            } else if map_lc.starts_with("a-s-") | map_lc.starts_with("s-a-") {
                let ch = map.chars().nth(4).unwrap();
                return Ok( KeyMap::AltShift(ch))
            } else if map_lc.starts_with("c-a-") | map_lc.starts_with("a-c-") {
                let ch = map.chars().nth(4).unwrap();
                return Ok( KeyMap::CtrlAlt(ch))
            }
        }
        return Err(LxConfigError::InvalidKeymap(map.to_string()));
    }
}

impl Serialize for KeyMap {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            KeyMap::Char(c) => s.serialize_newtype_variant("map", 0, "", &c),
            KeyMap::Ctrl(c) => s.serialize_newtype_variant("map", 0, "c-", &c),
            KeyMap::Alt(c) => s.serialize_newtype_variant("map", 0, "a-", &c),
            KeyMap::CtrlAlt(c) => s.serialize_newtype_variant("map", 0, "c-a-", &c),
            KeyMap::CtrlShift(c) => s.serialize_newtype_variant("map", 0, "c-s-", &c),
            KeyMap::AltShift(c) => s.serialize_newtype_variant("map", 0, "a-s-", &c),
        }

    }
}

impl<'de> Deserialize<'de> for KeyMap {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        unimplemented!();
        Ok(KeyMap::Char('c'))
    }
}

impl From<KeyMap> for KeyEvent {
    fn from(map: KeyMap) -> KeyEvent {
        match map {
            KeyMap::Char(c) => KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::Char(c) },
            KeyMap::Ctrl(c) => KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(c) },
            KeyMap::Alt(c) => KeyEvent { modifiers: KeyModifiers::ALT, code: KeyCode::Char(c) },
            KeyMap::CtrlShift(c) => KeyEvent { modifiers: KeyModifiers::CONTROL|KeyModifiers::SHIFT, code: KeyCode::Char(c) },
            KeyMap::AltShift(c) => KeyEvent{ modifiers: KeyModifiers::ALT|KeyModifiers::SHIFT, code: KeyCode::Char(c) },
            KeyMap::CtrlAlt(c) => KeyEvent{ modifiers: KeyModifiers::CONTROL|KeyModifiers::ALT, code: KeyCode::Char(c) }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyMaps {
    cursor: DirectionKeys,
    navigation: CycleKeys,
    edit_toggle: KeyMap,
    focus_toggle: KeyMap,
    overview_toggle: KeyMap,
}

impl Default for KeyMaps {
    fn default() -> Self {
        Self {
            navigation: CycleKeys {
                /* next: KeyMap::Char('n'),
                previous: KeyMap::Char('p'), */
                next: KeyMap::Char('\t'),
                previous: KeyMap::Ctrl('\t'),
            },
            cursor: DirectionKeys {
                up: KeyMap::Char('k'),
                down: KeyMap::Char('j'),
                left: KeyMap::Char('h'),
                right: KeyMap::Char('l'),
            },
            edit_toggle: KeyMap::Char('\n'),
            focus_toggle: KeyMap::Ctrl(' '),
            overview_toggle: KeyMap::Ctrl('o'),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DirectionKeys {
    up: KeyMap,
    down: KeyMap,
    left: KeyMap,
    right: KeyMap,
}
#[derive(Serialize, Deserialize)]
pub struct CycleKeys {
    next: KeyMap,
    previous: KeyMap
}

macro_rules! event {
    ($key:expr) => {
        Event { 0: $key }
    }
}
