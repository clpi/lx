pub mod prefix;
pub mod config;
pub mod error;
pub mod key;
pub mod types;
pub mod mode;
pub mod op;
pub mod ui;

use self::{
    mode::Mode,
    key::{EditKey, GlobalKey, GlobalPrefixKey, EditPrefixKey, map::*},
    prefix::{Prefix, MotionPre, WindowPre, SearchPre, TabPre, LeaderPre, BufferPre},
};
use std::io::{Stdout, Write, stdout};
use futures::{select, Stream, StreamExt, future::FutureExt};
use crossterm::{
    event::{self, Event, EventStream, KeyCode, KeyEvent, KeyModifiers, poll},
    cursor::{self, position}, style::{self, SetBackgroundColor, SetColors},
    terminal::{self, ClearType,  disable_raw_mode, enable_raw_mode},
    execute, queue, Result as CTResult,
};
use tui::{Terminal, backend::Backend};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Layout};
use tui::style::Style;
use tui::widgets::{Block, Borders, Paragraph};
use std::time::Duration;

pub use self::{
    error::{LxError, LxResult},
    types::Direction,
};


pub struct Lx<W: Write + Backend> {
    stdout: Stdout,
    prev_key: Option<KeyEvent>,
    prefix: Option<Prefix>,
    term: Terminal<W>,
    buf: Vec<String>,
    buf_idx: usize,
    mode: Mode,
    pos: (u16, u16),
    quit: bool,
}

impl Default for Lx<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        let mut so = stdout();
        let mut s = stdout();
        let backend = CrosstermBackend::new(s);
        let mut term = Terminal::new(backend)
            .expect("Could not initialize TUI");
        Self {
            stdout: so,
            quit: false,
            prefix: None,
            term,
            buf_idx: 0,
            buf: vec![String::new()],
            mode: Mode::insert(),
            pos: (0, 0),
            prev_key: None
        }
    }
}
impl Lx<CrosstermBackend<Stdout>> {
    pub fn run(&mut self) -> CTResult<()> {
        terminal::enable_raw_mode()?;
        self.term.clear()?;
        /* let mut reader = EventStream::new();
            let mut ev = reader.next().fuse(); */
        // self.init();
        execute!(self.stdout,
            event::EnableMouseCapture,
            terminal::EnterAlternateScreen
        )?;
        loop {
            queue!(
                &mut self.stdout,
                style::ResetColor,
                // terminal::Clear(ClearType::All),
                cursor::Show,
                // cursor::MoveTo(p.0, p.1),
            )?;
            let event = event::read()?;
            self.match_event(event)?;
            let mode = &self.mode;
            let bu = &self.buf;
            let st = format!("MODE: {}, PRE: {:?}, B: {}/{} Q: {}",
                &self.mode.to_string(),
                &self.prefix,
                &self.buf_idx,
                &self.buf.len(),
                &self.quit
                // &self.prev_key ==  &Some(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') })
            );
            let bu = self.buf[self.buf_idx].clone();
            let prev = self.prev_key.clone();
            self.term.draw(|r| {
                let s = r.size();
                let ch = Layout::default()
                    .direction(tui::layout::Direction::Vertical)
                    .margin(2)
                    .constraints([
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3)
                    ].as_ref())
                    .split(s);
                let pb = Paragraph::new(bu.to_string()).style(Style::default())
                    .alignment(tui::layout::Alignment::Left)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .style(Style::default())
                            .border_type(tui::widgets::BorderType::Plain)
                    );
                let p = Paragraph::new(st.to_string()).style(Style::default())
                    .alignment(tui::layout::Alignment::Center)
                    .block(Block::default().borders(Borders::ALL).style(Style::default())
                        .border_type(tui::widgets::BorderType::Plain));
                let debug = Paragraph::new(format!("{:?}",prev))
                    .style(Style::default())
                    .alignment(tui::layout::Alignment::Center)
                    .block(
                        Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default())
                        .border_type(tui::widgets::BorderType::Plain)
                    );
                r.render_widget(debug, ch[0]);
                r.render_widget(pb, ch[1]);
                r.render_widget(p, ch[2]);
            })?;
        }
    }

    fn write_char(&mut self, ch: char) {
        self.buf[self.buf_idx].push(ch);
        // println!("BUF: {}", self.buf);
    }

    fn match_event(&mut self, ev: Event) -> CTResult<()> {
        match ev {
            Event::Key(ke) => {self.match_key_event(ke)?;  },
            Event::Resize(x,y) => {
                // println!("{} {}", x, y);
            } ,
            _ => {},
        }
        Ok(())
    }
    fn match_leader_event(&mut self, ke: KeyEvent) -> CTResult<()> {
        // println!("LEADER ACTION!");
        if poll(Duration::from_millis(1_000))? {
            match ke {
                KeyEvent { code: KeyCode::Enter,.. } => {
                    // println!("LEADER+ENTER PRESSEED");
                },
                KeyEvent { code: KeyCode::Char(c), .. }  => match c {
                    '1'..='9' => { self.switch_buf(c as usize)?; },
                    'b' => { self.prefix = Some(Prefix::buffer()) },
                    'f' => { self.prefix = Some(Prefix::find()) },
                    'w' => { self.prefix = Some(Prefix::window()) },
                    '/' => { self.prefix = Some(Prefix::search()) },
                    't' => { self.prefix = Some(Prefix::tab()) },
                    _ =>  {}

                }
                KeyEvent { code, modifiers } => { },
                // println!("LEADER+{:?}+{:?}", code, modifiers);
            }
        } else {
            // println!("Leader timed out");
        }
        self.prefix = None;
        Ok(())
    }

    fn match_ctrl(&mut self, code: KeyCode) -> CTResult<()> {
        // println!("CTRL PRESSED");
        match code {
            KeyCode::Char('h') => {
                execute!(self.term.backend(), cursor::MoveLeft(1))?;
                self.pos.0 -= 1;
            }
            KeyCode::Char('j') => {
                execute!(self.stdout, cursor::MoveDown(1))?;
                self.pos.1 -= 1;
            }
            KeyCode::Char('k') => {
                execute!(self.stdout, cursor::MoveUp(1))?;
                self.pos.1 += 1;
            }
            KeyCode::Char('l') => {
                execute!(self.stdout, cursor::MoveRight(1))?;
                self.pos.0 += 1;
            }
            KeyCode::Char('q') => {
                self.quit = true;
                self.exit()?;
            }
            _ => {}
        }
        // println!("Cursor at : {:?} (says {:?})", position(), self.pos);
        execute!(self.stdout, cursor::SavePosition)?;
        Ok(())
    }
    fn match_shift(&mut self, code: KeyCode) -> CTResult<()> {
        // println!("SHIFT PRESSED");
        match code {
            KeyCode::Char('h') => {execute!(self.stdout, cursor::MoveLeft(1))?;}
            KeyCode::Char('j') => {execute!(self.stdout, cursor::MoveDown(1))?;}
            KeyCode::Char('k') => {execute!(self.stdout, cursor::MoveUp(1))?;}
            KeyCode::Char('l') => {execute!(self.stdout, cursor::MoveRight(1))?;}
            _ => {}
        }
        Ok(())
    }

    fn match_insert_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_ctrl(code)?;
            },
            KeyEvent { code: KeyCode::Char(c), .. } => self.write_char(c),
            /* KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
                self.match_shift(code)?;
            }, */
            KeyEvent { code, .. } => { self.match_key_code(code)?; },
        }
        Ok(())
    }
    fn match_overview_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {  },
            KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {  },
            _ => {}
        }
        Ok(())
    }
    fn match_command_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_ctrl(code)?;
            },
            /* KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
                self.match_shift(code)?;
            }, */
            KeyEvent { code, .. } => { self.match_key_code(code)?; },
        }
        Ok(())
    }
    fn match_edit_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        let leader = KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') };
        if self.prev_key == Some(leader) {
            self.match_leader_event(kv)?;
        } else {
            match kv {
                KeyEvent { modifiers: KeyModifiers::NONE, code } => match code {
                    KeyCode::Char('q') => { self.close_buf()?; },
                    KeyCode::Char('c') => { self.create_buf()?; },
                    _ => {  }
                },
                KeyEvent { modifiers: KeyModifiers::CONTROL, code } => match code {
                    KeyCode::Char('q') => { self.exit()?; },
                    _ => {  }
                },
                KeyEvent { modifiers: KeyModifiers::SHIFT, code } => match code {
                    KeyCode::Char('b') => {}
                    _ => {}
                }
                _ => {  }
            }
        }
        Ok(())
    }
    fn check_prefix(kv: KeyEvent) -> CTResult<()> {

    }
    fn match_prefix_key_event(&mut self, prefix: Prefix, kv: KeyEvent) -> CTResult<()> {
        match prefix {
            Prefix::Leader(_) => {  },
            _ => { }
        }
        Ok(())
    }

    fn match_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        if let Some(prefix) = Prefix::match_global_key(self.prev_key) {
            self.match_prefix_key_event(prefix, kv)?;
        } else if let Some(prefix) = Prefix::match_global_key(kv) {
            self.prefix = Some(prefix);
        } else if let Some(mode) = Mode::match_global_key(kv) {
            self.mode = Some(mode);
        } else {
            match self.mode {
                Mode::Insert(_) => {self.match_insert_key_event(kv)?;}
                Mode::Edit(_) => {self.match_edit_key_event(kv)?;}
                Mode::Command(_) => {self.match_command_key_event(kv)?;}
                Mode::Overview(_) => { self.match_overview_key_event(kv)?; }
            }
        }
        self.prev_key = Some(kv);
        Ok(())
    }
    fn match_key_code(&mut self, kc: KeyCode) -> CTResult<()> {
        match kc {
            KeyCode::Enter => {
                execute!(self.stdout, cursor::MoveToNextLine(1))?;
            },
            KeyCode::Backspace => {
                self.buf.pop();
                execute!(self.stdout, cursor::MoveLeft(1))?;
            },
            KeyCode::Char('h') | KeyCode::Left => {
                execute!(self.stdout, cursor::MoveLeft(1))?;

            }
            KeyCode::Char('j') | KeyCode::Down => {execute!(self.stdout, cursor::MoveDown(1))?;}
            KeyCode::Char('k') | KeyCode::Up => {execute!(self.stdout, cursor::MoveUp(1))?;}
            KeyCode::Char('l') | KeyCode::Right => {execute!(self.stdout, cursor::MoveRight(1))?;}
            KeyCode::PageUp => { execute!(self.stdout, cursor::MoveUp(8))?; }
            KeyCode::PageDown => { execute!(self.stdout, cursor::MoveDown(8))?; }
            KeyCode::Char('q') => { self.exit()?; }
            KeyCode::Esc => {},
            _ => {}
        }
        // println!("Cursor at : {:?}", position());
        Ok(())
    }
    fn create_buf(&mut self) -> CTResult<()> {
        self.buf.push(String::new());
        self.buf_idx = self.buf.len();
        Ok(())
    }
    fn close_buf(&mut self) -> CTResult<()> {
        self.buf.remove(self.buf_idx);
        if self.buf.len() == 0 {
            self.exit()?;
        } else {
            self.buf_idx -= 1;
        }
        Ok(())
    }
    fn switch_buf(&mut self, idx: usize) -> CTResult<()> {
        self.buf_idx = idx;
        Ok(())
    }
    fn exit(&mut self, ) -> CTResult<()> {
        execute!(self.stdout, event::DisableMouseCapture)?;
        disable_raw_mode()
    }
}

#[tokio::main]
async fn main() -> CTResult<()> {

    let mut t = Lx::default();
    t.run()?;
    Ok(())

}
