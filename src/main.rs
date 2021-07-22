pub mod prefix;
pub mod app;
pub mod config;
pub mod error;
pub mod key;
pub mod types;
pub mod mode;
pub mod op;
pub mod ui;

pub use self::{
    app::Lx,
    error::{LxError, LxResult},
    types::Direction,
    mode::Mode,
    key::{EditKey, GlobalKey, GlobalPrefixKey, EditPrefixKey},
    prefix::{Prefix, MotionPre, WindowPre, SearchPre, TabPre, LeaderPre, BufferPre},
};

#[tokio::main]
async fn main() -> LxResult<()> {

    let mut t = Lx::default();
    t.run()?;
    Ok(())

}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
    }

    pub struct TestStruct {
        field1: String,
        num: i32,
    }
}

