use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::config::{Theme, ColorTheme};
use crate::core::{load_todos, Status};
use crate::tui::forms::{FieldManager, FormRenderer, TerminalRunner, TodoFormData, TodoFormFields};

pub struct TodoEditor {
    fields: TodoFormFields,
    field_manager: FieldManager,
    todo_id: u32,
}

impl TodoEditor {
    pub fn new(todo_id: u32) -> Result<Self> {
        let mut editor = Self {
            fields: TodoFormFields::new(),
            field_manager: FieldManager::new(7), // 7 form fields
            todo_id,
        };

        editor.load_todo_data()?;
        editor.update_focus();
        Ok(editor)
    }

    fn load_todo_data(&mut self) -> Result<()> {
        // Use a blocking approach that's compatible with the current runtime
        use std::sync::Arc;
        use std::sync::Mutex;
        use std::thread;

        let result = Arc::new(Mutex::new(None));
        let result_clone = result.clone();

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let todos = rt.block_on(load_todos());
            *result_clone.lock().unwrap() = Some(todos);
        });

        handle.join().unwrap();
        let todo_list = result.lock().unwrap().take().unwrap()?;

        if let Some(todo) = todo_list.get_todo(self.todo_id) {
            self.fields.load_from_todo(todo);
        } else {
            anyhow::bail!("TODO with ID {} not found", self.todo_id);
        }

        Ok(())
    }

    fn update_focus(&mut self) {
        let focused_field = self.field_manager.focused_field();
        self.fields.set_field_focus(focused_field);
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        // Check for Enter in title field first (before field manager navigation)
        if let Event::Key(key) = event {
            if key.code == KeyCode::Enter && self.field_manager.focused_field() == 0 {
                // Enter pressed in title field (field index 0) - save and exit
                if let Err(e) = self.save_todo() {
                    eprintln!("Failed to save TODO: {}", e);
                } else {
                    return true;
                }
                return false;
            }
        }

        // Let the field manager handle navigation
        if self.field_manager.handle_navigation_event(event) {
            self.update_focus();
            return false;
        }

        // Let the focused component handle the event
        let focused_field = self.field_manager.focused_field();
        let handled = self.fields.handle_field_event(focused_field, event);

        if handled {
            return false;
        }

        // Handle global commands
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('x'))
                | (KeyModifiers::NONE, KeyCode::Esc) => {
                    // Auto-save on exit
                    if let Err(e) = self.save_todo() {
                        // TODO: Show error to user
                        eprintln!("Failed to save: {}", e);
                    }
                    return true;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                    if let Err(e) = self.save_todo() {
                        // TODO: Show error to user
                        eprintln!("Failed to save: {}", e);
                    } else {
                        return true;
                    }
                }
                (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                    // Quick archive shortcut
                    self.fields.status_select.set_selected(&Status::Archived);
                }
                _ => {}
            }
        }

        false
    }

    fn save_todo(&self) -> Result<()> {
        use std::sync::Arc;
        use std::sync::Mutex;
        use std::thread;

        let result = Arc::new(Mutex::new(None));
        let result_clone = result.clone();

        // Extract and validate form data
        let form_data = TodoFormData::from_fields(&self.fields)?;
        let todo_id = self.todo_id;

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(async {
                let mut todo_list = load_todos().await?;

                // Convert tags back to comma-separated string for update_todo API
                let tags_string = if form_data.tags.is_empty() {
                    None
                } else {
                    Some(form_data.tags.join(","))
                };

                todo_list.update_todo(
                    todo_id,
                    Some(format!("{:?}", form_data.status)),
                    Some(format!("{:?}", form_data.priority)),
                    tags_string,
                    form_data.category,
                    form_data.project,
                    form_data.notes,
                )?;

                // Update title separately since update_todo doesn't handle it
                if let Some(todo) = todo_list.get_todo_mut(todo_id) {
                    todo.title = form_data.title;
                }

                todo_list.save().await?;
                Ok::<(), anyhow::Error>(())
            });

            *result_clone.lock().unwrap() = Some(result);
        });

        handle.join().unwrap();
        let final_result = result.lock().unwrap().take().unwrap();
        final_result
    }

    fn draw(&mut self, f: &mut Frame) {
        let size = f.size();

        // Create a clean, minimalist layout with more compact spacing
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(1), // Header
                Constraint::Length(3), // Title (increased for underline)
                Constraint::Length(3), // Priority + Status (increased for underline)
                Constraint::Length(3), // Category + Project (increased for underline)
                Constraint::Length(3), // Tags (increased for underline)
                Constraint::Min(3),    // Notes
                Constraint::Length(1), // Help (minimal)
            ])
            .split(size);

        // Simple header with todo ID
        let header = format!("Edit TODO #{}", self.todo_id);
        let header_paragraph = ratatui::widgets::Paragraph::new(header).style(
            Style::default()
                .fg(Theme::primary())
                .add_modifier(Modifier::BOLD),
        );
        f.render_widget(header_paragraph, chunks[0]);

        // Title field with minimal styling
        FormRenderer::draw_minimal_input(f, chunks[1], &self.fields.title_input, "Title");

        // Priority and Status on same line, no boxes
        let priority_status_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        FormRenderer::draw_minimal_select(
            f,
            priority_status_chunks[0],
            &self.fields.priority_select,
            "Priority",
        );
        FormRenderer::draw_minimal_select(f, priority_status_chunks[1], &self.fields.status_select, "Status");

        // Category and Project on same line
        let category_project_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[3]);

        FormRenderer::draw_minimal_input(
            f,
            category_project_chunks[0],
            &self.fields.category_input,
            "Category",
        );
        FormRenderer::draw_minimal_input(
            f,
            category_project_chunks[1],
            &self.fields.project_input,
            "Project",
        );

        // Tags field
        FormRenderer::draw_minimal_input(f, chunks[4], &self.fields.tags_input, "Tags");

        // Notes with minimal styling
        FormRenderer::draw_minimal_textarea(f, chunks[5], &self.fields.notes_textarea);

        // Minimal help text
        let help_text = "⏎ Save & Exit (in title) • ⌃R Archive • ⌃S Save • ⌃X Exit (auto-saves) • ⇥ Next • ⇧⇥ Previous";
        let help_paragraph =
            ratatui::widgets::Paragraph::new(help_text).style(Style::default().fg(Theme::text_muted()));
        f.render_widget(help_paragraph, chunks[6]);
    }
}

pub async fn run_edit_todo(id: u32) -> Result<()> {
    let mut terminal = TerminalRunner::init()?;

    let mut editor = TodoEditor::new(id)?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| editor.draw(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            should_quit = editor.handle_event(&event);
        }
    }

    TerminalRunner::cleanup(terminal)?;
    Ok(())
}
