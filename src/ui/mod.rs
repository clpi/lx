use std::io::Write;
use crossterm::{event::{KeyCode, KeyEvent, KeyModifiers}, style::{Attribute, Color, SetForegroundColor, Stylize}, terminal::{self, SetTitle, EnableLineWrap, ScrollUp, ScrollDown}};
use tui::{
    Terminal, backend::{CrosstermBackend, Backend},
    layout::{Layout, Constraint},
    widgets::{Block, Borders, Paragraph},
    style::{Style, self},
};

use crate::{Lx, LxResult, Mode, mode::CommandMode};

pub fn draw_ui<W: Write + Backend>(app: &mut Lx<W>) -> LxResult<()> {
    let mode = &app.mode;
    let st = format!("MODE: {}, PRE: {:?}, B: {}/{} Q: {}",
        &mode.to_string(),
        &app.prefix,
        &app.buf_idx,
        &app.buf.len(),
        &app.quit
        // &terminal
        // &self.prev_key ==  &Some(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char(' ') })
    );
    let cursor = crossterm::cursor::position().expect("Could not get cursor");
    let bu = &app.buf[app.buf_idx].clone();
    let debug_str: String = match &app.mode {
        Mode::Command(_) =>  format!("CMD: {}", &app.cmd_buf ),
        _ =>  format!("POS: [{}, {}], P: {:?}", cursor.0, cursor.1,
            app.prev_keys.last().unwrap_or(&KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('q') })),
    };
    app.term.draw(|r| {
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
        let p = Paragraph::new(st.to_string())
            .style(Style::default())
            .alignment(tui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).style(Style::default())
                .border_type(tui::widgets::BorderType::Plain));
        let debug = Paragraph::new(debug_str.to_string())
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
    Ok(())
}
