use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};
use crossterm::event::{Event, KeyCode};
use crate::config::{Theme, ColorTheme};

pub struct Select<T> {
    options: Vec<(String, T)>,
    selected: usize,
    state: ListState,
    title: String,
    focused: bool,
}

impl<T: Clone> Select<T> {
    pub fn new(title: String, options: Vec<(String, T)>) -> Self {
        let mut state = ListState::default();
        if !options.is_empty() {
            state.select(Some(0));
        }
        
        Self {
            options,
            selected: 0,
            state,
            title,
            focused: false,
        }
    }

    pub fn selected_value(&self) -> Option<T> {
        self.options.get(self.selected).map(|(_, value)| value.clone())
    }

    pub fn set_selected(&mut self, value: &T) 
    where 
        T: PartialEq,
    {
        if let Some(index) = self.options.iter().position(|(_, v)| v == value) {
            self.selected = index;
            self.state.select(Some(index));
        }
    }

    pub fn get_selected_label(&self) -> Option<String> {
        self.options.get(self.selected).map(|(label, _)| label.clone())
    }

    pub fn handle_event(&mut self, event: &Event) -> bool {
        if !self.focused {
            return false;
        }

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => {
                    if self.selected > 0 {
                        self.selected -= 1;
                        self.state.select(Some(self.selected));
                    }
                    return true;
                }
                KeyCode::Down => {
                    if self.selected < self.options.len().saturating_sub(1) {
                        self.selected += 1;
                        self.state.select(Some(self.selected));
                    }
                    return true;
                }
                _ => {}
            }
        }
        false
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.options
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
