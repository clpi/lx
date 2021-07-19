use crate::types::Direction;

#[derive(Debug)]
pub enum Op {
    InsertChar(char),
    Buffer(BufferOp),
    Insert(InsertOp),
    Exit,
}
#[derive(Debug)]
pub enum BufferOp {
    SwitchTo(usize),
    Cycle(Direction),
    Close,
    CloseAll,
}
#[derive(Debug)]
pub enum InsertOp {
    InsertChar(char),
    Backspace,
    Delete,
    Return,
}
