
#[derive(Debug)]
pub enum Direction { Forwards, Backwards }
impl Default for Direction {
    fn default() -> Self {
        Self::Forwards
    }
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Self::Forwards => "FWD".into(),
            Self::Backwards => "BWD".into(),
        }
    }
}
