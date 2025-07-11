use anyhow::Result;
use crate::core::{Priority, Status};
use crate::tui::components::{Input, Select, TextArea};

/// Encapsulates all the form fields for a TODO
pub struct TodoFormFields {
    pub title_input: Input,
    pub priority_select: Select<Priority>,
    pub status_select: Select<Status>,
    pub category_input: Input,
    pub project_input: Input,
    pub tags_input: Input,
    pub notes_textarea: TextArea,
}

impl TodoFormFields {
    pub fn new() -> Self {
        let mut fields = Self {
            title_input: Input::new("Title"),
            priority_select: Select::new(
                "Priority".to_string(),
                vec![
                    ("P0 - Urgent".to_string(), Priority::P0),
                    ("P1 - Must have".to_string(), Priority::P1),
                    ("P2 - Should do".to_string(), Priority::P2),
                    ("P3 - Nice to have".to_string(), Priority::P3),
                    ("P4 - Wishlist".to_string(), Priority::P4),
                    ("P5 - Worth considering".to_string(), Priority::P5),
                ],
            ),
            status_select: Select::new(
                "Status".to_string(),
                vec![
                    ("Todo".to_string(), Status::Todo),
                    ("In Progress".to_string(), Status::InProgress),
                    ("Done".to_string(), Status::Done),
                    ("Archived".to_string(), Status::Archived),
                ],
            ),
            category_input: Input::new("Category"),
            project_input: Input::new("Project"),
            tags_input: Input::new("Tags (comma-separated)"),
            notes_textarea: TextArea::new("Notes".to_string()),
        };

        // Set defaults
        fields.priority_select.set_selected(&Priority::P2);
        fields.status_select.set_selected(&Status::Todo);

        fields
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title_input = self.title_input.with_value(title);
        self
    }

    pub fn load_from_todo(&mut self, todo: &crate::core::Todo) {
        self.title_input = self.title_input.clone().with_value(todo.title.clone());
        self.priority_select.set_selected(&todo.priority);
        self.status_select.set_selected(&todo.status);

        if let Some(category) = &todo.category {
            self.category_input = self.category_input.clone().with_value(category.clone());
        }

        if let Some(project) = &todo.project {
            self.project_input = self.project_input.clone().with_value(project.clone());
        }

        if !todo.tags.is_empty() {
            self.tags_input = self.tags_input.clone().with_value(todo.tags.join(","));
        }

        if let Some(notes) = &todo.notes {
            self.notes_textarea.set_content(notes.clone());
        }
    }

    /// Clear focus from all fields
    pub fn clear_focus(&mut self) {
        self.title_input.set_focused(false);
        self.priority_select.set_focused(false);
        self.status_select.set_focused(false);
        self.category_input.set_focused(false);
        self.project_input.set_focused(false);
        self.tags_input.set_focused(false);
        self.notes_textarea.set_focused(false);
    }

    /// Set focus on specific field by index
    pub fn set_field_focus(&mut self, field_index: usize) {
        self.clear_focus();
        match field_index {
            0 => self.title_input.set_focused(true),
            1 => self.priority_select.set_focused(true),
            2 => self.status_select.set_focused(true),
            3 => self.category_input.set_focused(true),
            4 => self.project_input.set_focused(true),
            5 => self.tags_input.set_focused(true),
            6 => self.notes_textarea.set_focused(true),
            _ => {} // Invalid index, keep all unfocused
        }
    }

    /// Handle event for focused field, returns true if event was handled
    pub fn handle_field_event(&mut self, field_index: usize, event: &crossterm::event::Event) -> bool {
        match field_index {
            0 => self.title_input.handle_event(event),
            1 => self.priority_select.handle_event(event),
            2 => self.status_select.handle_event(event),
            3 => self.category_input.handle_event(event),
            4 => self.project_input.handle_event(event),
            5 => self.tags_input.handle_event(event),
            6 => self.notes_textarea.handle_event(event),
            _ => false,
        }
    }
}

/// Form data extracted and validated from TodoFormFields
pub struct TodoFormData {
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub category: Option<String>,
    pub project: Option<String>,
    pub tags: Vec<String>,
    pub notes: Option<String>,
}

impl TodoFormData {
    /// Extract and validate form data from fields
    pub fn from_fields(fields: &TodoFormFields) -> Result<Self> {
        let title = fields.title_input.value.trim().to_string();
        crate::core::Todo::validate_title(&title)?;

        let priority = fields
            .priority_select
            .selected_value()
            .ok_or_else(|| anyhow::anyhow!("No priority selected"))?;

        let status = fields
            .status_select
            .selected_value()
            .ok_or_else(|| anyhow::anyhow!("No status selected"))?;

        let category = if fields.category_input.value.trim().is_empty() {
            None
        } else {
            let cat = fields.category_input.value.trim().to_string();
            crate::core::Todo::validate_category(&cat)?;
            Some(cat)
        };

        let project = if fields.project_input.value.trim().is_empty() {
            None
        } else {
            let proj = fields.project_input.value.trim().to_string();
            crate::core::Todo::validate_project(&proj)?;
            Some(proj)
        };

        let tags = if fields.tags_input.value.trim().is_empty() {
            Vec::new()
        } else {
            crate::core::Todo::validate_and_normalize_tags(&fields.tags_input.value)?
        };

        let notes = if fields.notes_textarea.content().trim().is_empty() {
            None
        } else {
            let notes_text = fields.notes_textarea.content().trim().to_string();
            crate::core::Todo::validate_notes(&notes_text)?;
            Some(notes_text)
        };

        Ok(Self {
            title,
            priority,
            status,
            category,
            project,
            tags,
            notes,
        })
    }
}
