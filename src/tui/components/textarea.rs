use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crate::config::{Theme, ColorTheme};

pub struct TextArea {
    content: String,
    cursor_position: usize,
    title: String,
    focused: bool,
    scroll: u16,
}

impl TextArea {
    pub fn new(title: String) -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
            title,
            focused: false,
            scroll: 0,
        }
    }

    pub fn with_content(title: String, content: String) -> Self {
        let cursor_position = content.len();
        Self {
            content,
            cursor_position,
            title,
            focused: false,
            scroll: 0,
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.cursor_position = self.content.len();
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    // Allow character input unless it's a control sequence we want to handle elsewhere
                    if !modifiers.contains(KeyModifiers::CONTROL) {
                        self.content.insert(self.cursor_position, *c);
                        self.cursor_position += 1;
                        return true;
                    }
                }
                KeyCode::Backspace if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.content.remove(self.cursor_position);
                    }
                    return true;
                }
                KeyCode::Delete if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position < self.content.len() {
                        self.content.remove(self.cursor_position);
                    }
                    return true;
                }
                KeyCode::Left if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                    }
                    return true;
                }
                KeyCode::Right if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position < self.content.len() {
                        self.cursor_position += 1;
                    }
                    return true;
                }
                KeyCode::Home if *modifiers == KeyModifiers::NONE => {
                    self.cursor_position = 0;
                    return true;
                }
                KeyCode::End if *modifiers == KeyModifiers::NONE => {
                    self.cursor_position = self.content.len();
                    return true;
                }
                KeyCode::Enter if *modifiers == KeyModifiers::NONE => {
                    self.content.insert(self.cursor_position, '\n');
                    self.cursor_position += 1;
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let border_style = if self.focused {
            Style::default().fg(Theme::focused())
        } else {
            Style::default()
        };

        let paragraph = Paragraph::new(self.content.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.as_str())
                    .border_style(border_style)
            )
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0));

        f.render_widget(paragraph, area);
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
