use crossterm::event::{Event, KeyCode, KeyModifiers};

/// Manages field focus and navigation for forms
pub struct FieldManager {
    focused_field: usize,
    field_count: usize,
}

impl FieldManager {
    pub fn new(field_count: usize) -> Self {
        Self {
            focused_field: 0,
            field_count,
        }
    }

    pub fn focused_field(&self) -> usize {
        self.focused_field
    }

    pub fn focus_next_field(&mut self) {
        self.focused_field = (self.focused_field + 1) % self.field_count;
    }

    pub fn focus_previous_field(&mut self) {
        self.focused_field = if self.focused_field == 0 {
            self.field_count - 1
        } else {
            self.focused_field - 1
        };
    }

    pub fn set_focused_field(&mut self, field_index: usize) {
        if field_index < self.field_count {
            self.focused_field = field_index;
        }
    }

    /// Handle navigation events, returns true if this was a navigation event
    pub fn handle_navigation_event(&mut self, event: &Event) -> bool {
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Tab) => {
                    self.focus_next_field();
                    true
                }
                (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                    self.focus_previous_field();
                    true
                }
                _ => false,
            }
        } else {
            false
        }
    }
}
