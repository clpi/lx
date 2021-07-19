use std::io::{Stdout, Write, stdout};
use tui::{ Terminal, backend::{CrosstermBackend, Backend}};

use super::{
    ui,
    prefix::Prefix,
    mode::Mode,
};
use crossterm::{
    event::{self, Event, EventStream, KeyCode, KeyEvent, KeyModifiers, poll},
    cursor::{self, position}, style::{self, SetBackgroundColor, SetColors},
    terminal::{self, ClearType,  disable_raw_mode, enable_raw_mode},
    execute, queue, Result as CTResult,
};
use std::time::Duration;

// TODO make wrapper type for key event / event type
pub struct Lx<W: Write + Backend> {
    pub prev_keys: Vec<KeyEvent>,
    pub prefix: Option<Prefix>,
    pub term: Terminal<W>,
    pub buf: Vec<String>,
    pub buf_idx: usize,
    pub mode: Mode,
    pub pos: (u16, u16),
    pub quit: bool,
}

impl Default for Lx<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let term = Terminal::new(backend)
            .expect("Could not initialize TUI");
        let mut prev_keys = Vec::with_capacity(5);
        prev_keys.push(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') });
        Self {
            quit: false,
            prefix: None,
            term,
            buf_idx: 0,
            buf: vec![String::new()],
            mode: Mode::insert(),
            pos: (0, 0),
            prev_keys: Vec::with_capacity(4),
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
        execute!(self.term.backend_mut(),
            event::EnableMouseCapture,
            terminal::EnterAlternateScreen,
            terminal::EnableLineWrap,
            terminal::SetTitle("lx editor"),
        )?;
        loop {
            queue!(
                &mut self.term.backend_mut(),
                style::ResetColor,
                cursor::Show,
            )?;
            let event = event::read()?;
            self.match_event(event)?;
            ui::draw_ui(self).expect("Error drawing tui");
            // self.term.flush()?;
        }
    }

    /// STEP 1
        /// Step 1.1: Check if prev keypress triggered prefix
    fn match_key_event(&mut self, kv: KeyEvent) -> CTResult<()> {
        let prev_key = *self.prev_keys.last()
            .unwrap_or(&KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') });

        if let Some(prefix) = Prefix::match_global_key(prev_key) {
            self.match_prefix_key_event(prefix, kv)?;

        } else if let Some(prefix) = Prefix::match_global_key(kv) {
            self.prefix = Some(prefix);

        } else if let Some(mode) = Mode::match_key(kv) {
            self.mode = mode;
        } else {
            match &self.mode {
                Mode::Insert(_ctx) => {self.match_insert_key_event(kv)?;}
                Mode::Edit(_ctx) => {self.match_edit_key_event(kv)?;}
                Mode::Command(_ctx) => {self.match_command_key_event(kv)?;}
                Mode::Overview(_ctx) => { self.match_overview_key_event(kv)?; }
            }
        }
        self.prev_keys.push(kv);
        Ok(())
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
                KeyEvent { code, modifiers } => {

                },
                // println!("LEADER+{:?}+{:?}", code, modifiers);
            }
        } else {
            // println!("Leader timed out");
        }
        self.prefix = None;
        Ok(())
    }

    fn match_insert_ctrl(&mut self, code: KeyCode) -> CTResult<()> {
        // println!("CTRL PRESSED");
        match code {
            KeyCode::Char('h') => {
                execute!(self.term.backend_mut(), cursor::MoveLeft(1))?;
                self.pos.0 -= 1;
            }
            KeyCode::Char('j') => {
                execute!(self.term.backend_mut(), cursor::MoveDown(1))?;
                self.pos.1 -= 1;
            }
            KeyCode::Char('k') => {
                execute!(self.term.backend_mut(), cursor::MoveUp(1))?;
                self.pos.1 += 1;
            }
            KeyCode::Char('l') => {
                execute!(self.term.backend_mut(), cursor::MoveRight(1))?;
                self.pos.0 += 1;
            }
            KeyCode::Char('q') => {
                self.quit = true;
                self.exit()?;
            }
            _ => {}
        }
        // println!("Cursor at : {:?} (says {:?})", position(), self.pos);
        execute!(self.term.backend_mut(), cursor::SavePosition)?;
        Ok(())
    }
    fn match_shift(&mut self, code: KeyCode) -> CTResult<()> {
        // println!("SHIFT PRESSED");
        match code {
            KeyCode::Char('h') => {execute!(self.term.backend_mut(), cursor::MoveLeft(1))?;}
            KeyCode::Char('j') => {execute!(self.term.backend_mut(), cursor::MoveDown(1))?;}
            KeyCode::Char('k') => {execute!(self.term.backend_mut(), cursor::MoveUp(1))?;}
            KeyCode::Char('l') => {execute!(self.term.backend_mut(), cursor::MoveRight(1))?;}
            _ => {}
        }
        Ok(())
    }

    fn match_insert_key_event(&mut self, kv: KeyEvent, ) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::PageUp } => {
                execute!(self.term.backend_mut(), terminal::ScrollUp(3))?;
            },
            KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::PageDown } => {
                execute!(self.term.backend_mut(), terminal::ScrollDown(3))?;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') } => {
                self.exit()?;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_insert_ctrl(code)?;
            },
            KeyEvent { code: KeyCode::Char(c), .. } => self.write_char(c),
            /* KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
                self.match_shift(code)?;
            }, */
            KeyEvent { code, .. } => { self.match_key_code(code)?; },
        }
        Ok(())
    }
    fn match_overview_key_event(&mut self, kv: KeyEvent, ) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') } => {
                self.exit()?;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {  },
            KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {  },
            _ => {}
        }
        Ok(())
    }
    fn match_command_key_event(&mut self, kv: KeyEvent, ) -> CTResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') } => {
                self.exit()?;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_insert_ctrl(code)?;
            },
            /* KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
                self.match_shift(code)?;
            }, */
            KeyEvent { code, .. } => { self.match_key_code(code)?; },
        }
        Ok(())
    }
    fn match_edit_key_event(&mut self, kv: KeyEvent, ) -> CTResult<()> {
        let leader = KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') };
        if self.prev_keys.last() == Some(&leader) {
            self.match_leader_event(kv)?;
        } else {
            match kv {
                KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') } => {
                    self.exit()?;
                },
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
    fn match_prefix_key_event(&mut self, prefix: Prefix, kv: KeyEvent) -> CTResult<()> {
        match prefix {
            Prefix::Leader(_) => {  },
            _ => { }
        }
        Ok(())
    }

    fn match_key_code(&mut self, kc: KeyCode) -> CTResult<()> {
        match kc {
            KeyCode::Enter => {
                execute!(self.term.backend_mut(), cursor::MoveToNextLine(1))?;
            },
            KeyCode::Backspace => {
                self.buf.pop();
                execute!(self.term.backend_mut(), cursor::MoveLeft(1))?;
            },
            KeyCode::Char('h') | KeyCode::Left => {
                execute!(self.term.backend_mut(), cursor::MoveLeft(1))?;

            }
            KeyCode::Char('j') | KeyCode::Down => {execute!(self.term.backend_mut(), cursor::MoveDown(1))?;}
            KeyCode::Char('k') | KeyCode::Up => {execute!(self.term.backend_mut(), cursor::MoveUp(1))?;}
            KeyCode::Char('l') | KeyCode::Right => {execute!(self.term.backend_mut(), cursor::MoveRight(1))?;}
            KeyCode::PageUp => { execute!(self.term.backend_mut(), cursor::MoveUp(8))?; }
            KeyCode::PageDown => { execute!(self.term.backend_mut(), cursor::MoveDown(8))?; }
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
        execute!(self.term.backend_mut(), event::DisableMouseCapture)?;
        disable_raw_mode()
    }
}
