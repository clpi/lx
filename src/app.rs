use std::io::{Stdout, Write, stdout};
use tui::{ Terminal, buffer::{Buffer, Cell}, backend::{CrosstermBackend, Backend}};

use super::{
    ui,
    prefix::Prefix,
    mode::Mode,
    LxResult,
};
use crossterm::{Result as CTResult, cursor::{self, CursorShape, position}, event::{self, Event, EventStream, KeyCode, KeyEvent, KeyModifiers, poll}, execute, queue, style::{self, SetBackgroundColor, SetColors}, terminal::{self, ClearType,  disable_raw_mode, enable_raw_mode}};
use std::time::Duration;

// TODO make wrapper type for key event / event type
// TODO use tui buffer type for buffers
pub struct Lx<W: Write + Backend> {
    pub prev_keys: Vec<KeyEvent>,
    pub prefix: Option<Prefix>,
    pub term: Terminal<W>,
    pub buf: Vec<String>,
    pub buf_idx: usize,
    pub cmd_buf: String,
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
            cmd_buf: String::new(),
            buf: vec![String::new()],
            mode: Mode::insert(),
            pos: (0, 0),
            prev_keys: Vec::with_capacity(4),
        }
    }
}
impl Lx<CrosstermBackend<Stdout>> {

    pub fn run(&mut self) -> LxResult<()> {
        terminal::enable_raw_mode()?;
        self.term.clear()?;
        /* let mut reader = EventStream::new();
            let mut ev = reader.next().fuse(); */
        // self.init();
        execute!(self.term.backend_mut(),
            event::EnableMouseCapture,
            terminal::EnterAlternateScreen,
            terminal::EnableLineWrap,
            cursor::EnableBlinking,
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

    pub fn exec_cmd(&mut self) -> LxResult<()> {
        match self.cmd_buf.as_str() {
            "quit" => { self.quit = true; },
            "command" => { self.mode = Mode::command(); },
            "insert" => { self.mode = Mode::insert(); },
            "edit" => { self.mode = Mode::edit(); },
            "overview" => { self.mode = Mode::overview(); },
            _ => {  }
        }
        Ok(())
    }
    /// STEP 1
        /// Step 1.1: Check if prev keypress triggered prefix
    fn match_key_event(&mut self, kv: KeyEvent) -> LxResult<()> {
        let prev_key = *self.prev_keys.last()
            .unwrap_or(&KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') });

        if let Some(prefix) = Prefix::match_global_key(prev_key) {
            self.match_prefix_key_event(prefix, kv)?;

        } else if let Some(prefix) = Prefix::match_global_key(kv) {
            self.prefix = Some(prefix);

        } else if let Some(mode) = self.mode.match_key(kv) {
            self.mode_switch(mode)?;
        } else {
            match &self.mode {
                Mode::Insert(_ctx) => {self.match_insert_key_event(kv)?;}
                Mode::Edit(_ctx) => {self.match_edit_key_event(kv)?;}
                Mode::Command(_ctx) => {self.match_command_key_event(kv)?;}
                Mode::Overview(_ctx) => { self.match_overview_key_event(kv)?; }
            }
        }
        self.prev_keys.push(kv);
        if self.quit {
            self.exit()?;
        }
        Ok(())
    }
    fn mode_switch(&mut self, mode: Mode) -> LxResult<()> {
        if self.mode != mode {
            match self.mode {
                Mode::Insert(_) | Mode::Edit(_) => {
                    execute!(self.term.backend_mut(),
                        cursor::SavePosition)?;
                },
                _ => {  }
            }
            match mode {
                Mode::Insert(_) => {
                    execute!(self.term.backend_mut(),
                        cursor::RestorePosition,
                        cursor::SetCursorShape(CursorShape::Line),
                    )?;
                },
                Mode::Command(_) => {
                    execute!(self.term.backend_mut(),
                        cursor::MoveToRow(terminal::size()?.1-1),
                        cursor::SetCursorShape(CursorShape::Line),)?;
                },
                Mode::Edit(_) => {
                    execute!(self.term.backend_mut(),
                        cursor::SetCursorShape(CursorShape::Block),
                        cursor::RestorePosition)?;
                },
                Mode::Overview(_) => {
                    execute!(self.term.backend_mut(),
                        cursor::SetCursorShape(CursorShape::Block))?;
                }
            }
            self.mode = mode;
        }
        Ok(())
    }
    fn write_char(&mut self, ch: char) {
        self.buf[self.buf_idx].push(ch);
    }
    fn write_cmd_char(&mut self, ch: char) {
        self.cmd_buf.push(ch);
    }

    fn match_event(&mut self, ev: Event) -> LxResult<()> {
        match ev {
            Event::Key(ke) => {self.match_key_event(ke)?;  },
            Event::Resize(x,y) => {
            } ,
            _ => {},
        }
        Ok(())
    }
    fn match_leader_event(&mut self, ke: KeyEvent) -> CTResult<()> {
        if poll(Duration::from_millis(1_000))? {
            match ke {
                KeyEvent { code: KeyCode::Enter,.. } => {
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
            }
        } else {
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
            }
            _ => {}
        }
        execute!(self.term.backend_mut(), cursor::SavePosition)?;
        Ok(())
    }
    fn match_shift(&mut self, code: KeyCode) -> CTResult<()> {
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
                self.quit = true;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_insert_ctrl(code)?;
            },
            KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::Backspace } => {
                self.buf[self.buf_idx].pop();
                execute!(self.term.backend_mut(), cursor::MoveLeft(1))?;
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
                self.quit = true;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {  },
            KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {  },
            _ => {}
        }
        Ok(())
    }
    fn match_command_key_event(&mut self, kv: KeyEvent, ) -> LxResult<()> {
        match kv {
            KeyEvent { modifiers: KeyModifiers::NONE, code: KeyCode::Enter  } => {
                self.exec_cmd()?;
                self.mode_switch(Mode::edit())?;
            }
            KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') } => {
                self.quit = true;
            },
            KeyEvent { modifiers: KeyModifiers::CONTROL, code } => {
                self.match_insert_ctrl(code)?;
            },
            /* KeyEvent { modifiers: KeyModifiers::SHIFT, code } => {
                self.match_shift(code)?;
            }, */
            KeyEvent { code: KeyCode::Char(c), .. } => {
                self.write_cmd_char(c);
            },
            _ => {  },
        }
        Ok(())
    }
    fn match_edit_key_event(&mut self, kv: KeyEvent, ) -> CTResult<()> {
        let leader = KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') };
        if self.prev_keys.last() == Some(&leader) {
            self.match_leader_event(kv)?;
        } else {
            match kv {
                KeyEvent { modifiers: KeyModifiers::CONTROL, code } => match code {
                    KeyCode::Char('q') => { self.quit = true; },
                    _ => {  }
                },
                KeyEvent { modifiers: KeyModifiers::NONE, code } => match code {
                    KeyCode::Char('q') => { self.close_buf()?; },
                    KeyCode::Char('c') => { self.create_buf()?; },
                    KeyCode::Char('n') => { execute!(self.term.backend_mut(), cursor::MoveToNextLine(1))?; },
                    KeyCode::Char('p') => { execute!(self.term.backend_mut(), cursor::MoveToPreviousLine(1))?; },
                    KeyCode::Char('j') | KeyCode::Down  => { execute!(self.term.backend_mut(), cursor::MoveDown(1))?; },
                    KeyCode::Char('h') | KeyCode::Left => { execute!(self.term.backend_mut(), cursor::MoveLeft(1))?; },
                    KeyCode::Char('k') | KeyCode::Up => { execute!(self.term.backend_mut(), cursor::MoveUp(1))?; },
                    KeyCode::Char('l') | KeyCode::Right => { execute!(self.term.backend_mut(), cursor::MoveRight(1))?; },
                    KeyCode::PageUp => { execute!(self.term.backend_mut(), terminal::ScrollUp(3))?; },
                    KeyCode::PageDown => { execute!(self.term.backend_mut(), terminal::ScrollDown(3))?; },
                    _ => {  }
                }
                KeyEvent { modifiers, code } => {}

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
            KeyCode::Char('q') => { self.quit = true; }
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
             self.quit = true;
        } else {
            self.buf_idx -= 1;
        }
        Ok(())
    }
    fn switch_buf(&mut self, idx: usize) -> CTResult<()> {
        self.buf_idx = idx;
        Ok(())
    }
    pub fn exit(&mut self, ) -> CTResult<()> {
        execute!(self.term.backend_mut(),
            event::DisableMouseCapture,
            terminal::LeaveAlternateScreen,
            terminal::Clear(ClearType::All),
            )?;
        terminal::disable_raw_mode();
        Ok(())
    }
}
