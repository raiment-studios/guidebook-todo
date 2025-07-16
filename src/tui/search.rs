use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};

use crate::config::{ColorTheme, Theme};
use crate::core::{get_git_status, load_todos, GitStatus, Priority, Status, Todo};
use crate::tui::components::Input;
use crate::tui::forms::{FormRenderer, TerminalRunner};

pub struct TodoSearcher {
    search_input: Input,
    filtered_todos: Vec<Todo>,
    all_todos: Vec<Todo>,
    selected_index: usize,
    list_state: ListState,
    show_help: bool,
    focus_on_search: bool, // true = search input, false = results list
    git_status: GitStatus,
}

impl TodoSearcher {
    pub async fn new(query: Option<String>) -> Result<Self> {
        let todo_list = load_todos().await?;
        let all_todos = todo_list.todos.clone();

        // Get git status
        let git_status = get_git_status().unwrap_or_else(|_| GitStatus {
            has_changes: false,
            pretty_path: "unknown".to_string(),
            status_message: "unavailable".to_string(),
        });

        // Apply initial filtering (archived and old done todos)
        let initial_filtered = all_todos
            .iter()
            .filter(|todo| Self::should_include_todo(todo))
            .cloned()
            .collect();

        let mut searcher = Self {
            search_input: Input::new("Search"),
            filtered_todos: initial_filtered,
            all_todos,
            selected_index: 0,
            list_state: ListState::default(),
            show_help: false,
            focus_on_search: true, // Start with search input focused
            git_status,
        };

        // Pre-fill search if provided
        if let Some(query) = query {
            searcher.search_input = searcher.search_input.clone().with_value(query);
            searcher.filter_todos();
        }

        // Set initial focus state
        searcher.search_input.set_focused(searcher.focus_on_search);
        searcher.update_selection();
        Ok(searcher)
    }

    fn filter_todos(&mut self) {
        let query = self.search_input.value.to_lowercase();

        if query.is_empty() {
            self.filtered_todos = self
                .all_todos
                .iter()
                .filter(|todo| Self::should_include_todo(todo))
                .cloned()
                .collect();
        } else {
            self.filtered_todos = self
                .all_todos
                .iter()
                .filter(|todo| {
                    // Apply base filtering first
                    if !Self::should_include_todo(todo) {
                        return false;
                    }

                    // Advanced search syntax
                    if query.starts_with("#") {
                        // Tag filtering: #tag
                        let tag = &query[1..];
                        return todo.tags.iter().any(|t| t.to_lowercase().contains(tag));
                    } else if query.starts_with("@") {
                        // Category filtering: @category
                        let category = &query[1..];
                        return todo
                            .category
                            .as_ref()
                            .map_or(false, |c| c.to_lowercase().contains(category));
                    } else if query.starts_with("!") {
                        // Status filtering: !status
                        let status = &query[1..];
                        return format!("{:?}", todo.status).to_lowercase().contains(status);
                    } else if query.starts_with("p") && query.len() == 2 {
                        // Priority filtering: p0, p1, etc.
                        if let Some(priority_char) = query.chars().nth(1) {
                            if priority_char.is_ascii_digit() {
                                let expected_priority = format!("p{}", priority_char);
                                return format!("{}", todo.priority).to_lowercase()
                                    == expected_priority;
                            }
                        }
                    }

                    // Default fuzzy search across all fields
                    todo.title.to_lowercase().contains(&query)
                        || todo
                            .notes
                            .as_ref()
                            .map_or(false, |n| n.to_lowercase().contains(&query))
                        || todo
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query))
                        || todo
                            .category
                            .as_ref()
                            .map_or(false, |c| c.to_lowercase().contains(&query))
                        || todo
                            .project
                            .as_ref()
                            .map_or(false, |p| p.to_lowercase().contains(&query))
                })
                .cloned()
                .collect();
        }

        // Reset selection
        self.selected_index = 0;
        self.update_selection();
    }

    fn update_selection(&mut self) {
        if self.filtered_todos.is_empty() || self.focus_on_search {
            self.list_state.select(None);
        } else {
            self.selected_index = self.selected_index.min(self.filtered_todos.len() - 1);
            self.list_state.select(Some(self.selected_index));
        }
    }

    fn move_selection_up(&mut self) {
        if !self.filtered_todos.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.update_selection();
        }
    }

    fn move_selection_down(&mut self) {
        if !self.filtered_todos.is_empty() && self.selected_index < self.filtered_todos.len() - 1 {
            self.selected_index += 1;
            self.update_selection();
        }
    }

    fn get_selected_todo(&self) -> Option<&Todo> {
        self.filtered_todos.get(self.selected_index)
    }

    fn get_status_icon(&self, todo: &Todo) -> &'static str {
        match todo.status {
            crate::core::Status::Todo => "T",
            crate::core::Status::InProgress => "W", // Work in progress
            crate::core::Status::Done => "D",
            crate::core::Status::Archived => "A",
        }
    }

    fn handle_event(&mut self, event: &Event) -> Result<Option<u32>> {
        // Handle global navigation first
        if let Event::Key(key) = event {
            match (key.modifiers, key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('x')) => return Ok(Some(0)), // Signal exit
                (KeyModifiers::NONE, KeyCode::Esc) => return Ok(Some(0)),          // Signal exit
                (KeyModifiers::CONTROL, KeyCode::Char('a')) => return Ok(Some(5000)), // Signal add todo
                (KeyModifiers::NONE, KeyCode::F(1)) => {
                    self.show_help = !self.show_help;
                    return Ok(None);
                }
                (KeyModifiers::NONE, KeyCode::Char('/')) => {
                    // Always focus search input
                    self.focus_on_search = true;
                    self.search_input.set_focused(true);
                    return Ok(None);
                }
                _ => {}
            }
        }

        // Handle events based on current focus
        if self.focus_on_search {
            // Search input has focus
            if let Event::Key(key) = event {
                match (key.modifiers, key.code) {
                    (KeyModifiers::NONE, KeyCode::Down) => {
                        // Switch focus to results list
                        if !self.filtered_todos.is_empty() {
                            self.focus_on_search = false;
                            self.search_input.set_focused(false);
                            self.selected_index = 0;
                            self.update_selection();
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        // If search has focus and Enter is pressed, go to first result
                        if !self.filtered_todos.is_empty() {
                            self.focus_on_search = false;
                            self.search_input.set_focused(false);
                            self.selected_index = 0;
                            self.update_selection();
                        }
                        return Ok(None);
                    }
                    _ => {
                        // Handle search input events (including typing =, -, +)
                        if self.search_input.handle_event(event) {
                            self.filter_todos();
                        }
                        return Ok(None);
                    }
                }
            }
        } else {
            // Results list has focus
            if let Event::Key(key) = event {
                match (key.modifiers, key.code) {
                    (KeyModifiers::NONE, KeyCode::Up) => {
                        if self.selected_index == 0 {
                            // Switch focus back to search input
                            self.focus_on_search = true;
                            self.search_input.set_focused(true);
                            self.list_state.select(None);
                        } else {
                            // Move up in results
                            self.move_selection_up();
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::NONE, KeyCode::Down) => {
                        self.move_selection_down();
                        return Ok(None);
                    }
                    (KeyModifiers::NONE, KeyCode::Enter) => {
                        if let Some(todo) = self.get_selected_todo() {
                            return Ok(Some(todo.id)); // Signal edit todo
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::NONE, KeyCode::Char('='))
                    | (KeyModifiers::NONE, KeyCode::Char('+')) => {
                        // Increase priority (make it higher) - only when results have focus
                        if let Some(todo) = self.get_selected_todo() {
                            return Ok(Some(1000 + todo.id)); // Signal priority increase
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::NONE, KeyCode::Char('-')) => {
                        // Decrease priority (make it lower) - only when results have focus
                        if let Some(todo) = self.get_selected_todo() {
                            return Ok(Some(2000 + todo.id)); // Signal priority decrease
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
                        // Archive todo - only when results have focus
                        if let Some(todo) = self.get_selected_todo() {
                            return Ok(Some(3000 + todo.id)); // Signal archive todo
                        }
                        return Ok(None);
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                        // Mark todo as done - only when results have focus
                        if let Some(todo) = self.get_selected_todo() {
                            return Ok(Some(4000 + todo.id)); // Signal mark done
                        }
                        return Ok(None);
                    }
                    _ => {}
                }
            }
        }

        Ok(None)
    }

    async fn reload_todos(&mut self) -> Result<()> {
        let todo_list = load_todos().await?;
        self.all_todos = todo_list.todos.clone();
        self.filter_todos();
        Ok(())
    }

    async fn reload_todos_preserving_selection(&mut self, preserve_todo_id: u32) -> Result<()> {
        let todo_list = load_todos().await?;
        self.all_todos = todo_list.todos.clone();
        self.filter_todos();

        // Find the TODO with the preserved ID and update selection to it
        if let Some(position) = self
            .filtered_todos
            .iter()
            .position(|todo| todo.id == preserve_todo_id)
        {
            self.selected_index = position;
            self.update_selection();
        }

        Ok(())
    }

    async fn change_todo_priority(&mut self, todo_id: u32, increase: bool) -> Result<()> {
        let mut todo_list = load_todos().await?;

        if let Some(todo) = todo_list.todos.iter_mut().find(|t| t.id == todo_id) {
            let new_priority = if increase {
                // Higher priority means lower number (P0 > P1 > P2 > P3 > P4 > P5)
                match todo.priority {
                    Priority::P5 => Priority::P4,
                    Priority::P4 => Priority::P3,
                    Priority::P3 => Priority::P2,
                    Priority::P2 => Priority::P1,
                    Priority::P1 => Priority::P0,
                    Priority::P0 => Priority::P0, // Already highest
                }
            } else {
                // Lower priority means higher number
                match todo.priority {
                    Priority::P0 => Priority::P1,
                    Priority::P1 => Priority::P2,
                    Priority::P2 => Priority::P3,
                    Priority::P3 => Priority::P4,
                    Priority::P4 => Priority::P5,
                    Priority::P5 => Priority::P5, // Already lowest
                }
            };

            todo.priority = new_priority;
            todo_list.save().await?;

            // Reload todos while preserving focus on the changed item
            self.reload_todos_preserving_selection(todo_id).await?;
        }

        Ok(())
    }

    async fn archive_todo(&mut self, todo_id: u32) -> Result<()> {
        let mut todo_list = load_todos().await?;

        if let Some(todo) = todo_list.todos.iter_mut().find(|t| t.id == todo_id) {
            todo.status = Status::Archived;
            todo_list.save().await?;

            // Reload todos and remove focus from archived item (since it won't be visible)
            self.reload_todos().await?;
        }

        Ok(())
    }

    async fn mark_todo_done(&mut self, todo_id: u32) -> Result<()> {
        let mut todo_list = load_todos().await?;

        if let Some(todo) = todo_list.todos.iter_mut().find(|t| t.id == todo_id) {
            todo.status = Status::Done;
            if todo.finished_date.is_none() {
                todo.finished_date = Some(chrono::Local::now());
            }
            todo_list.save().await?;

            // Reload todos while preserving focus on the changed item
            self.reload_todos_preserving_selection(todo_id).await?;
        }

        Ok(())
    }

    fn draw(&mut self, f: &mut Frame) {
        let size = f.size();

        // Create minimalist layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3), // Header + git status
                Constraint::Length(3), // Search input + focus indicator
                Constraint::Min(3),    // Results list
                Constraint::Length(1), // Help
            ])
            .split(size);

        // Header with git status
        self.draw_header_with_git_status(f, chunks[0]);

        // Search input with minimal styling
        let search_area = Rect::new(chunks[1].x, chunks[1].y, chunks[1].width, 2);
        FormRenderer::draw_minimal_search_input(f, search_area, &self.search_input);

        // Add focus indicator
        let indicator_area = Rect::new(chunks[1].x, chunks[1].y + 2, chunks[1].width, 1);
        if self.focus_on_search {
            let focus_indicator =
                ratatui::widgets::Paragraph::new("● Search focused - ↓ to navigate results")
                    .style(Style::default().fg(Theme::accent()));
            f.render_widget(focus_indicator, indicator_area);
        } else {
            let focus_indicator =
                ratatui::widgets::Paragraph::new("● Results focused - ↑ to return to search")
                    .style(Style::default().fg(Theme::primary()));
            f.render_widget(focus_indicator, indicator_area);
        }

        // Results list with clean styling
        self.draw_minimal_results_list(f, chunks[2]);

        // Minimal help text
        let help_text = if self.show_help {
            "Search: #tag @category !status p0-p5 • ↑↓ Navigate • ⏎ Edit • +/= Higher Priority • - Lower Priority • ⌃R Archive • ⌃D Done • ⌃A Add TODO • F1 Toggle Help"
        } else if self.focus_on_search {
            "Type to search • ↓/⏎ Navigate to results • / Refocus • ⌃A Add TODO • F1 Help • ⌃X Exit"
        } else {
            "↑↓ Navigate • ⏎ Edit • +/= Higher Priority • - Lower Priority • ⌃R Archive • ⌃D Done • ⌃A Add TODO • F1 Help • ⌃X Exit"
        };

        let help_paragraph = ratatui::widgets::Paragraph::new(help_text)
            .style(Style::default().fg(Theme::text_muted()));
        f.render_widget(help_paragraph, chunks[3]);
    }

    fn draw_header_with_git_status(&self, f: &mut Frame, area: Rect) {
        // Split the header area into title and git status
        let header_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Title
                Constraint::Length(1), // Git status
                Constraint::Length(1), // Separator
            ])
            .split(area);

        // Title
        let title_paragraph = ratatui::widgets::Paragraph::new("Search TODOs").style(
            Style::default()
                .fg(Theme::primary())
                .add_modifier(Modifier::BOLD),
        );
        f.render_widget(title_paragraph, header_chunks[0]);

        // Git status
        let git_status_color = if self.git_status.has_changes {
            Theme::warning()
        } else {
            Theme::text_muted()
        };

        let git_status_text = format!(
            "data dir: {} • git status: {}",
            self.git_status.pretty_path, self.git_status.status_message
        );

        let git_status_paragraph = ratatui::widgets::Paragraph::new(git_status_text)
            .style(Style::default().fg(git_status_color));
        f.render_widget(git_status_paragraph, header_chunks[1]);

        // Separator line
        let separator = "─".repeat(area.width as usize);
        let separator_paragraph = ratatui::widgets::Paragraph::new(separator)
            .style(Style::default().fg(Theme::text_muted()));
        f.render_widget(separator_paragraph, header_chunks[2]);
    }

    fn draw_minimal_results_list(&mut self, f: &mut Frame, area: Rect) {
        if area.height < 1 {
            return;
        }

        // Results count header
        let results_header = format!("{} results found", self.filtered_todos.len());
        let header_paragraph = ratatui::widgets::Paragraph::new(results_header)
            .style(Style::default().fg(Theme::text_secondary()));
        f.render_widget(header_paragraph, Rect::new(area.x, area.y, area.width, 1));

        // Results list without borders
        let list_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);

        let items: Vec<ListItem> = self
            .filtered_todos
            .iter()
            .enumerate()
            .map(|(index, todo)| {
                let icon = self.get_status_icon(todo);
                let category = todo.category.as_deref().unwrap_or("general");

                // Calculate available width for title
                // Format: "001 P1 S │ general  │ title"
                // Fixed parts: 3 (id) + 1 (space) + 2 (priority) + 1 (space) + 1 (status) + 1 (space) + 1 (│) + 1 (space) + 8 (category) + 1 (space) + 1 (│) + 1 (space) = 22
                let fixed_width = 22;
                let available_width = (area.width as usize).saturating_sub(fixed_width);

                let title = if todo.title.len() > available_width && available_width > 3 {
                    format!("{}...", &todo.title[..available_width.saturating_sub(3)])
                } else if available_width <= 3 {
                    "...".to_string()
                } else {
                    todo.title.clone()
                };

                // Get priority color
                let priority_color = match todo.priority {
                    crate::core::Priority::P0 => Theme::priority_urgent(),
                    crate::core::Priority::P1 => Theme::priority_high(),
                    crate::core::Priority::P2 => Theme::priority_medium(),
                    crate::core::Priority::P3 | crate::core::Priority::P4 => Theme::priority_low(),
                    crate::core::Priority::P5 => Theme::priority_wishlist(),
                };

                // Get status color
                let status_color = match todo.status {
                    crate::core::Status::Todo => Theme::status_todo(),
                    crate::core::Status::InProgress => Theme::status_in_progress(),
                    crate::core::Status::Done => Theme::status_done(),
                    crate::core::Status::Archived => Theme::status_archived(),
                };

                // Create styled spans
                let id_span = Span::styled(
                    format!("{:03} ", todo.id),
                    Style::default().fg(Theme::text_disabled()),
                );
                let priority_span = Span::styled(
                    format!("{:2} ", todo.priority),
                    Style::default().fg(priority_color),
                );
                let status_span =
                    Span::styled(format!("{} ", icon), Style::default().fg(status_color));
                let separator1_span = Span::styled("│ ", Style::default().fg(Theme::text_muted()));
                let category_span = Span::styled(
                    format!("{:8} ", category),
                    Style::default().fg(Theme::text_secondary()),
                );
                let separator2_span = Span::styled("│ ", Style::default().fg(Theme::text_muted()));
                let title_span = Span::styled(
                    format!("{}", title),
                    Style::default().fg(Theme::text_primary()),
                );

                let line = Line::from(vec![
                    id_span,
                    priority_span,
                    status_span,
                    separator1_span,
                    category_span,
                    separator2_span,
                    title_span,
                ]);

                let item_style = if Some(index) == self.list_state.selected() {
                    Style::default()
                        .bg(Theme::selected())
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                ListItem::new(line).style(item_style)
            })
            .collect();

        let list = List::new(items);

        f.render_stateful_widget(list, list_area, &mut self.list_state);
    }

    /// Helper method to determine if a TODO should be included in the filtered results
    /// Excludes archived TODOs and Done TODOs older than 3 seconds
    fn should_include_todo(todo: &Todo) -> bool {
        // Filter out archived TODOs
        if todo.status == Status::Archived {
            return false;
        }

        // Filter out Done TODOs that are older than 3 seconds
        if todo.status == Status::Done {
            if let Some(finished_date) = todo.finished_date {
                let now = chrono::Local::now();
                let duration = now.signed_duration_since(finished_date);
                if duration.num_seconds() > 3 {
                    return false;
                }
            }
        }

        true
    }
}

pub async fn run_search_todo(query: Option<String>) -> Result<()> {
    let mut terminal = TerminalRunner::init()?;

    let mut searcher = TodoSearcher::new(query).await?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|f| searcher.draw(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            match searcher.handle_event(&event)? {
                Some(0) => should_quit = true, // Exit signal
                Some(5000) => {
                    // Add TODO signal - exit search, run add, then return
                    TerminalRunner::leave_tui_mode(terminal)?;

                    // Run add TODO in a separate async context
                    use crate::tui::add_todo::run_add_todo;
                    if let Err(e) = run_add_todo(None).await {
                        eprintln!("Failed to add TODO: {}", e);
                    }

                    // Recreate terminal for search
                    terminal = TerminalRunner::recreate()?;

                    // Reload todos
                    searcher.reload_todos().await?;
                }
                Some(todo_id) if todo_id >= 4000 => {
                    // Mark done signal (4000 + todo_id)
                    let actual_id = todo_id - 4000;
                    if let Err(e) = searcher.mark_todo_done(actual_id).await {
                        eprintln!("Failed to mark TODO as done: {}", e);
                    }
                }
                Some(todo_id) if todo_id >= 3000 => {
                    // Archive signal (3000 + todo_id)
                    let actual_id = todo_id - 3000;
                    if let Err(e) = searcher.archive_todo(actual_id).await {
                        eprintln!("Failed to archive TODO: {}", e);
                    }
                }
                Some(todo_id) if todo_id >= 2000 => {
                    // Priority decrease signal (2000 + todo_id)
                    let actual_id = todo_id - 2000;
                    if let Err(e) = searcher.change_todo_priority(actual_id, false).await {
                        eprintln!("Failed to decrease priority: {}", e);
                    }
                }
                Some(todo_id) if todo_id >= 1000 => {
                    // Priority increase signal (1000 + todo_id)
                    let actual_id = todo_id - 1000;
                    if let Err(e) = searcher.change_todo_priority(actual_id, true).await {
                        eprintln!("Failed to increase priority: {}", e);
                    }
                }
                Some(todo_id) => {
                    // Edit todo - exit search, run edit, then return
                    TerminalRunner::leave_tui_mode(terminal)?;

                    // Run edit in a separate async context
                    use crate::tui::edit_todo::run_edit_todo;
                    if let Err(e) = run_edit_todo(todo_id).await {
                        eprintln!("Failed to edit TODO: {}", e);
                    }

                    // Recreate terminal for search
                    terminal = TerminalRunner::recreate()?;

                    // Reload todos
                    searcher.reload_todos().await?;
                }
                None => {} // Continue
            }
        }
    }

    TerminalRunner::cleanup(terminal)?;
    Ok(())
}
