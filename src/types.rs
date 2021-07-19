#[derive(Debug, PartialEq)]
pub enum CursorDirection { Up, Down, Left, Right }

impl ToString for CursorDirection {
    fn to_string(&self) -> String {
        match self {
            Self::Up => "U".into(),
            Self::Down => "D".into(),
            Self::Left => "L".into(),
            Self::Right => "R".into(),
        }
    }

}

#[derive(Debug, PartialEq)]
pub enum Direction { Next, Prev }

impl Default for Direction {
    fn default() -> Self {
        Self::Next
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Self::Next => "->".into(),
            Self::Prev => "<-".into(),
        }
    }
}
