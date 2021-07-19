use crossterm::event::{KeyModifiers, KeyCode, KeyEvent};

/* pub trait PrefixKey {

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
    fn match_sub_key(ke: KeyEvent) -> Option<Self> {

    }
}
 */
/// A key combination that works in all modes. Checked first. Used primarily for prefix and
/// mode-switching key events.
pub trait GlobalKey: Default {

    fn key() -> KeyEvent;
    fn match_key(ke: KeyEvent) -> Option<Self> {
        if Self::key() == ke {
            Some(Self::default())
        } else { None }
    }
}

/// A key combination that works exclusively in Edit mode.
pub trait EditKey: Default {
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
