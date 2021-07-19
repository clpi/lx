use std::{io::Write, path::PathBuf};
use crossterm::execute;
use tui::backend::Backend;
use crate::{Lx, error::{LxResult, LxError}, mode::Mode, prefix::FindTarget, types::Direction};

pub trait Operation: Default {
    fn exec<W: Write>(&self, w: W) -> LxResult<()>;

    fn exec_app<W: Write + Backend>(&self, app: &mut Lx<W>) -> LxResult<()> {
        app.mode = Mode::edit();
        Ok(())
    }
}

/* #[derive(Debug)]
pub enum Op {
    InsertChar(char),
    Buffer(BufferOp),
    Insert(InsertOp),
    Exit,
} */
#[derive(Debug)]
pub enum InsertOp {
    InsertChar(char),
    Backspace(usize),
    Delete(usize),
    Return(usize),
    Nothing
}

#[derive(Debug)]
pub enum ModeOp {
    ToggleInsert,
    Insert,
    Edit,
    Overview,
    Command,
}
#[derive(Debug)]
pub enum CommandOp {
    InsertChar(char),
    Backspace(usize),
    Delete(usize),
    Enter,
    Nothing
}
#[derive(Debug)]
pub enum EditOp {
    InsertChar(char),
    CutChar(Direction, usize),
    CutLine,
    Paste(Direction),
    Backspace(usize),
    MoveLine(Direction, usize),
    InsertLine(Direction, usize),
    Delete(usize),
    Return(usize),
    Nothing,
}
#[derive(Debug)]
pub enum GlobalOp {
    Find(FindTarget),
    OpenFile(PathBuf),
    NewBuffer,
    InsertChar(char),
    Backspace(usize),
    Delete,
    Return,
    Nothing
}
#[derive(Debug)]
pub enum SearchOp {
    Nothing,
    SearchInBuffer(Direction, String),
    ReplaceCharInBuffer(Direction, String),
    SearchInDir(String),
    ReplaceInDir(String),
}
#[derive(Debug)]
pub enum MotionOp {
    Nothing,
    SkipWord(Direction, usize),
    SkipWordEnd(Direction, usize),
    GoToChar(Direction, char, usize),
    GoFindChar(Direction, char, usize),
    GoToBufferEnd(Direction),
}

impl Default for EditOp { fn default() -> Self { EditOp::Nothing } }
impl Default for CommandOp { fn default() -> Self { CommandOp::Nothing } }
impl Default for GlobalOp { fn default() -> Self { GlobalOp::Nothing } }
impl Default for InsertOp { fn default() -> Self { InsertOp::Nothing }}
impl Default for SearchOp { fn default() -> Self { SearchOp::Nothing }}
impl Default for MotionOp { fn default() -> Self { MotionOp::Nothing }}
impl Default for ModeOp {
    fn default() -> Self {
        ModeOp::Insert
    }
}

impl Operation for EditOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for GlobalOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            Self::Return => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for InsertOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for SearchOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for MotionOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for CommandOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        match self {
            Self::Nothing => {},
            _ => {  }
        }
        Ok(())
    }
}
impl Operation for ModeOp {
    fn exec<W: Write>(&self, w: W) -> LxResult<()> {
        Ok(())
    }
    fn exec_app<W: Write + Backend>(&self, lx: &mut Lx<W>) -> LxResult<()> {
        match self {
            Self::Insert => { lx.mode = Mode::insert(); },
            Self::Edit => { lx.mode = Mode::edit(); },
            Self::Overview => { lx.mode = Mode::overview(); },
            Self::Command => { lx.mode = Mode::command(); },
            _ => {  }
        }
        Ok(())
    }
}
