use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::config::{ColorTheme, Theme};
use crate::core::load_todos;
use crate::tui::forms::{FieldManager, FormRenderer, TerminalRunner, TodoFormData, TodoFormFields};

pub struct TodoCreator {
    fields: TodoFormFields,
    field_manager: FieldManager,
}

impl TodoCreator {
    pub fn new(title: Option<String>) -> Self {
        let mut fields = TodoFormFields::new();

        // Pre-fill title if provided
        if let Some(title) = title {
            fields = fields.with_title(title);
        }

        let mut creator = Self {
            fields,
            field_manager: FieldManager::new(7), // 7 form fields
        };

        creator.update_focus();
        creator
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
                (KeyModifiers::CONTROL, KeyCode::Char('x')) => {
                    // Auto-save on exit
                    if let Err(e) = self.save_todo() {
                        eprintln!("Failed to save TODO: {}", e);
                    }
                    return true;
                }
                (KeyModifiers::NONE, KeyCode::Esc) => {
                    // Auto-save on exit
                    if let Err(e) = self.save_todo() {
                        eprintln!("Failed to save TODO: {}", e);
                    }
                    return true;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                    // Manual save and exit
                    if let Err(e) = self.save_todo() {
                        eprintln!("Failed to save TODO: {}", e);
                    } else {
                        return true;
                    }
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

        let handle = thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let result = rt.block_on(async {
                let mut todo_list = load_todos().await?;

                let mut todo = todo_list.create_todo(form_data.title);
                todo.priority = form_data.priority;
                todo.status = form_data.status;
                todo.category = form_data.category;
                todo.project = form_data.project;
                todo.tags = form_data.tags;
                todo.notes = form_data.notes;

                todo_list.add_todo(todo);
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
        let main_chunks = Layout::default()
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

        // Simple header
        let header_paragraph = ratatui::widgets::Paragraph::new("Add New TODO").style(
            Style::default()
                .fg(Theme::primary())
                .add_modifier(Modifier::BOLD),
        );
        f.render_widget(header_paragraph, main_chunks[0]);

        // Title field with minimal styling
        FormRenderer::draw_minimal_input(f, main_chunks[1], &self.fields.title_input, "Title");

        // Priority and Status on same line, no boxes
        let priority_status_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[2]);

        FormRenderer::draw_minimal_select(
            f,
            priority_status_chunks[0],
            &self.fields.priority_select,
            "Priority",
        );
        FormRenderer::draw_minimal_select(
            f,
            priority_status_chunks[1],
            &self.fields.status_select,
            "Status",
        );

        // Category and Project on same line
        let category_project_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_chunks[3]);

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
        FormRenderer::draw_minimal_input(f, main_chunks[4], &self.fields.tags_input, "Tags");

        // Notes with minimal styling
        FormRenderer::draw_minimal_textarea(f, main_chunks[5], &self.fields.notes_textarea);

        // Minimal help text
        let help_text = "⏎ Save & Exit (in title) • ⌃S Save & Exit • ⌃X Exit (auto-saves) • Esc Exit (auto-saves) • ⇥ Next • ⇧⇥ Previous";
        let help_paragraph = ratatui::widgets::Paragraph::new(help_text)
            .style(Style::default().fg(Theme::text_muted()));
        f.render_widget(help_paragraph, main_chunks[6]);
    }
}

pub async fn run_add_todo(title: Option<String>) -> Result<()> {
    let mut terminal = TerminalRunner::init()?;

    let mut creator = TodoCreator::new(title);
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| creator.draw(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            should_quit = creator.handle_event(&event);
        }
    }

    TerminalRunner::cleanup(terminal)?;
    Ok(())
}
