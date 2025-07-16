use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crate::config::{Theme, ColorTheme};

#[derive(Debug, Clone)]
pub struct Input {
    pub value: String,
    pub cursor_position: usize,
    pub focused: bool,
    pub placeholder: String,
}

impl Input {
    pub fn new(placeholder: &str) -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
            focused: false,
            placeholder: placeholder.to_string(),
        }
    }

    pub fn with_value(mut self, value: String) -> Self {
        self.cursor_position = value.len();
        self.value = value;
        self
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        if let Event::Key(KeyEvent { code, modifiers, .. }) = event {
            match code {
                KeyCode::Char(c) => {
                    if *modifiers == KeyModifiers::CONTROL {
                        match c {
                            'a' => {
                                self.cursor_position = 0;
                                return true;
                            }
                            'e' => {
                                self.cursor_position = self.value.len();
                                return true;
                            }
                            _ => {}
                        }
                    } else {
                        // Allow character input with SHIFT, ALT, or no modifiers
                        self.value.insert(self.cursor_position, *c);
                        self.cursor_position += 1;
                        return true;
                    }
                }
                KeyCode::Backspace if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position > 0 {
                        self.cursor_position -= 1;
                        self.value.remove(self.cursor_position);
                    }
                    return true;
                }
                KeyCode::Delete if *modifiers == KeyModifiers::NONE => {
                    if self.cursor_position < self.value.len() {
                        self.value.remove(self.cursor_position);
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
                    if self.cursor_position < self.value.len() {
                        self.cursor_position += 1;
                    }
                    return true;
                }
                KeyCode::Home if *modifiers == KeyModifiers::NONE => {
                    self.cursor_position = 0;
                    return true;
                }
                KeyCode::End if *modifiers == KeyModifiers::NONE => {
                    self.cursor_position = self.value.len();
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn draw(&self, f: &mut Frame, area: Rect, title: &str) {
        let display_text = if self.value.is_empty() && !self.focused {
            &self.placeholder
        } else {
            &self.value
        };

        let style = if self.focused {
            Style::default().fg(Theme::focused())
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(style);

        let paragraph = Paragraph::new(display_text.as_str())
            .block(block)
            .style(if self.value.is_empty() && !self.focused {
                Style::default().fg(Theme::text_disabled())
            } else {
                Style::default()
            });

        f.render_widget(paragraph, area);

        // Render cursor if focused
        if self.focused {
            let cursor_x = area.x + 1 + self.cursor_position as u16;
            let cursor_y = area.y + 1;
            if cursor_x < area.x + area.width - 1 && cursor_y < area.y + area.height - 1 {
                f.set_cursor(cursor_x, cursor_y);
            }
        }
    }
}
