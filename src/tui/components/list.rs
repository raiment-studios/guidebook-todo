use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crossterm::event::{Event, KeyCode};
use crate::config::{Theme, ColorTheme};

pub struct TodoList<T> {
    items: Vec<(String, T)>,
    state: ListState,
    title: String,
    focused: bool,
}

impl<T: Clone> TodoList<T> {
    pub fn new(title: String, items: Vec<(String, T)>) -> Self {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        
        Self {
            items,
            state,
            title,
            focused: false,
        }
    }

    pub fn selected_value(&self) -> Option<T> {
        if let Some(selected) = self.state.selected() {
            self.items.get(selected).map(|(_, value)| value.clone())
        } else {
            None
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => {
                    if let Some(selected) = self.state.selected() {
                        if selected > 0 {
                            self.state.select(Some(selected - 1));
                        }
                    }
                    return true;
                }
                KeyCode::Down => {
                    if let Some(selected) = self.state.selected() {
                        if selected < self.items.len().saturating_sub(1) {
                            self.state.select(Some(selected + 1));
                        }
                    }
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|(label, _)| ListItem::new(label.as_str()))
            .collect();

        let border_style = if self.focused {
            Style::default().fg(Theme::focused())
        } else {
            Style::default()
        };

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(self.title.as_str())
                    .border_style(border_style)
            )
            .highlight_style(Style::default().bg(Theme::selected()));

        f.render_stateful_widget(list, area, &mut self.state);
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }
}
