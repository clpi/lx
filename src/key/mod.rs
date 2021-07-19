pub mod maps;

use crossterm::event::{KeyModifiers, KeyCode, KeyEvent};
use crate::op::{Operation, GlobalOp, EditOp, InsertOp, CommandOp};

pub trait EditPrefixKey: Default {
    type Op: Operation + Default;

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
    /// Matches the key event immediately following the triggering of this prefix to the
    /// corresponding operation. ONLY applies for prefix keys.
    fn match_key_op(ke: KeyEvent) -> Option<Self::Op> {
        match ke {
            KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(' ') }=>  { return Some(Self::Op::default()); }
            _ => { return Some(Self::Op::default()); }
        }
    }
}

/// A key combination that works in all modes. Checked first. Used primarily for prefix and
/// mode-switching key events.
pub trait GlobalKey: Default {
    type Op: Operation + Default;

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
}

pub trait GlobalPrefixKey: Default {
    type Op: Operation + Default;

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
    /// Matches the key event immediately following the triggering of this prefix to the
    /// corresponding operation
    fn match_key_op(ke: KeyEvent) -> Option<Self::Op> {
        match ke {
            KeyEvent{modifiers: KeyModifiers::NONE, code: KeyCode::Char(' ') }=>  { return Some(Self::Op::default()); }
            _ => { return Some(Self::Op::default()); }
        }
    }
}

/// A key combination that works exclusively in Edit mode.
pub trait EditKey: Default {
    type Op: Operation + Default;
    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
}
/* pub trait CommandKey {

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
}

/// A key combination that works exclusively in Insert mode. Likely few combinations which don't
/// overlap with Global Key combinations
pub trait InsertKey {

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
} */
