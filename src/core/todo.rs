use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    #[serde(default)]
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub project: Option<String>,
    pub created_date: DateTime<Local>,
    pub finished_date: Option<DateTime<Local>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Priority {
    P0, // Urgent - work on this right now
    P1, // Must have
    P2, // Should do
    P3, // Nice to have
    P4, // Wishlist
    P5, // Worth considering
}

impl Default for Priority {
    fn default() -> Self {
        Priority::P2
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::P0 => write!(f, "P0"),
            Priority::P1 => write!(f, "P1"),
            Priority::P2 => write!(f, "P2"),
            Priority::P3 => write!(f, "P3"),
            Priority::P4 => write!(f, "P4"),
            Priority::P5 => write!(f, "P5"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Archived,
}

impl Default for Status {
    fn default() -> Self {
        Status::Todo
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::InProgress => write!(f, "InProgress"),
            Status::Done => write!(f, "Done"),
            Status::Archived => write!(f, "Archived"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub next_id: u32,
    #[serde(default)]
    pub todos: Vec<Todo>,
}

impl Default for TodoList {
    fn default() -> Self {
        TodoList {
            next_id: 1,
            todos: Vec::new(),
        }
    }
}

impl Todo {
    pub fn new(title: String) -> Self {
        Todo {
            id: 0, // Will be set by TodoList
            title,
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            category: None,
            project: None,
            created_date: Local::now(),
            finished_date: None,
            notes: None,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, Status::Todo | Status::InProgress)
    }

    pub fn priority_value(&self) -> u8 {
        match self.priority {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
            Priority::P5 => 5,
        }
    }

    /// Validates a TODO title according to SPEC (max 200 chars, non-empty)
    pub fn validate_title(title: &str) -> Result<()> {
        let trimmed = title.trim();
        if trimmed.is_empty() {
            anyhow::bail!("Title cannot be empty");
        }
        if trimmed.len() > 200 {
            anyhow::bail!(
                "Title cannot exceed 200 characters (current: {})",
                trimmed.len()
            );
        }
        Ok(())
    }

    /// Validates a category according to SPEC (max 50 chars)
    pub fn validate_category(category: &str) -> Result<()> {
        if category.len() > 50 {
            anyhow::bail!(
                "Category cannot exceed 50 characters (current: {})",
                category.len()
            );
        }
        Ok(())
    }

    /// Validates a project according to SPEC (max 100 chars)
    pub fn validate_project(project: &str) -> Result<()> {
        if project.len() > 100 {
            anyhow::bail!(
                "Project cannot exceed 100 characters (current: {})",
                project.len()
            );
        }
        Ok(())
    }

    /// Validates notes according to SPEC (max 2000 chars)
    pub fn validate_notes(notes: &str) -> Result<()> {
        if notes.len() > 2000 {
            anyhow::bail!(
                "Notes cannot exceed 2000 characters (current: {})",
                notes.len()
            );
        }
        Ok(())
    }

    /// Normalizes and validates tags according to SPEC (lowercase, no spaces)
    pub fn validate_and_normalize_tags(tags_input: &str) -> Result<Vec<String>> {
        let tags: Vec<String> = tags_input
            .split(',')
            .map(|s| s.trim().to_lowercase())
            .filter(|s| !s.is_empty())
            .map(|s| s.replace(' ', "_")) // Replace spaces with underscores
            .collect();

        for tag in &tags {
            if tag.len() > 30 {
                anyhow::bail!("Tag '{}' cannot exceed 30 characters", tag);
            }
            if tag.contains(' ') {
                anyhow::bail!("Tag '{}' cannot contain spaces (use underscores)", tag);
            }
        }

        Ok(tags)
    }

    /// Creates a new TODO with validation
    pub fn new_validated(title: String) -> Result<Self> {
        Self::validate_title(&title)?;

        Ok(Todo {
            id: 0, // Will be set by TodoList
            title: title.trim().to_string(),
            priority: Priority::default(),
            status: Status::default(),
            tags: Vec::new(),
            category: None,
            project: None,
            created_date: Local::now(),
            finished_date: None,
            notes: None,
        })
    }

    /// Updates the TODO with validation
    pub fn update_with_validation(
        &mut self,
        title: Option<String>,
        category: Option<String>,
        project: Option<String>,
        notes: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Result<()> {
        if let Some(title) = title {
            Self::validate_title(&title)?;
            self.title = title.trim().to_string();
        }

        if let Some(category) = category {
            Self::validate_category(&category)?;
            self.category = if category.trim().is_empty() {
                None
            } else {
                Some(category.trim().to_string())
            };
        }

        if let Some(project) = project {
            Self::validate_project(&project)?;
            self.project = if project.trim().is_empty() {
                None
            } else {
                Some(project.trim().to_string())
            };
        }

        if let Some(notes) = notes {
            Self::validate_notes(&notes)?;
            self.notes = if notes.trim().is_empty() {
                None
            } else {
                Some(notes.trim().to_string())
            };
        }

        if let Some(tags) = tags {
            self.tags = tags;
        }

        Ok(())
    }
}

impl TodoList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_todo(&mut self, title: String) -> Todo {
        let mut todo = Todo::new(title);
        todo.id = self.next_id;
        self.next_id += 1;
        todo
    }

    pub fn add_todo(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn get_todo(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_todo_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id == id)
    }

    pub fn delete_todo(&mut self, id: u32) -> Result<()> {
        let pos = self.todos.iter().position(|todo| todo.id == id);
        match pos {
            Some(index) => {
                self.todos.remove(index);
                Ok(())
            }
            None => anyhow::bail!("TODO with ID {} not found", id),
        }
    }

    pub fn delete_by_category(&mut self, category: &str) -> usize {
        let initial_len = self.todos.len();
        self.todos
            .retain(|todo| todo.category.as_ref().map_or(true, |cat| cat != category));
        initial_len - self.todos.len()
    }

    pub fn delete_by_status(&mut self, status_str: &str) -> Result<usize> {
        let status = parse_status(status_str)?;
        let initial_len = self.todos.len();
        self.todos.retain(|todo| todo.status != status);
        Ok(initial_len - self.todos.len())
    }

    pub fn update_todo(
        &mut self,
        id: u32,
        status: Option<String>,
        priority: Option<String>,
        tags: Option<String>,
        category: Option<String>,
        project: Option<String>,
        notes: Option<String>,
    ) -> Result<()> {
        let todo = self
            .get_todo_mut(id)
            .ok_or_else(|| anyhow::anyhow!("TODO with ID {} not found", id))?;

        if let Some(status_str) = status {
            let new_status = parse_status(&status_str)?;
            let old_status = todo.status.clone();
            todo.status = new_status;

            // Set finished_date when marking as Done
            if matches!(todo.status, Status::Done)
                && matches!(old_status, Status::Todo | Status::InProgress)
            {
                todo.finished_date = Some(Local::now());
            }
            // Clear finished_date when unmarking
            else if matches!(todo.status, Status::Todo | Status::InProgress)
                && matches!(old_status, Status::Done)
            {
                todo.finished_date = None;
            }
        }

        if let Some(priority_str) = priority {
            todo.priority = parse_priority(&priority_str)?;
        }

        if let Some(tags_str) = tags {
            update_tags(&mut todo.tags, &tags_str);
        }

        if let Some(cat) = category {
            todo.category = if cat.is_empty() { None } else { Some(cat) };
        }

        if let Some(proj) = project {
            todo.project = if proj.is_empty() { None } else { Some(proj) };
        }

        if let Some(note) = notes {
            todo.notes = if note.is_empty() { None } else { Some(note) };
        }

        Ok(())
    }

    pub fn show_stats(&self) {
        let total = self.todos.len();
        let mut status_counts = HashMap::new();
        let mut priority_counts = HashMap::new();
        let mut category_counts = HashMap::new();

        for todo in &self.todos {
            *status_counts.entry(todo.status.clone()).or_insert(0) += 1;
            *priority_counts.entry(todo.priority.clone()).or_insert(0) += 1;
            if let Some(ref category) = todo.category {
                *category_counts.entry(category.clone()).or_insert(0) += 1;
            }
        }

        println!("📊 TODO Statistics");
        println!("==================");
        println!("Total TODOs: {}", total);
        println!();

        println!("By Status:");
        for (status, count) in status_counts {
            println!("  {}: {}", status, count);
        }
        println!();

        println!("By Priority:");
        for (priority, count) in priority_counts {
            println!("  {}: {}", priority, count);
        }
        println!();

        if !category_counts.is_empty() {
            println!("By Category:");
            for (category, count) in category_counts {
                println!("  {}: {}", category, count);
            }
        }
    }
}

pub fn parse_status(status_str: &str) -> Result<Status> {
    match status_str.to_lowercase().as_str() {
        "todo" => Ok(Status::Todo),
        "inprogress" | "in-progress" | "in_progress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        "archived" => Ok(Status::Archived),
        _ => anyhow::bail!(
            "Invalid status: {}. Valid values: todo, inprogress, done, archived",
            status_str
        ),
    }
}

pub fn parse_priority(priority_str: &str) -> Result<Priority> {
    match priority_str.to_lowercase().as_str() {
        "p0" => Ok(Priority::P0),
        "p1" => Ok(Priority::P1),
        "p2" => Ok(Priority::P2),
        "p3" => Ok(Priority::P3),
        "p4" => Ok(Priority::P4),
        "p5" => Ok(Priority::P5),
        _ => anyhow::bail!(
            "Invalid priority: {}. Valid values: p0, p1, p2, p3, p4, p5",
            priority_str
        ),
    }
}

pub fn update_tags(tags: &mut Vec<String>, tags_str: &str) {
    for tag_part in tags_str.split(',') {
        let tag_part = tag_part.trim();
        if tag_part.starts_with('+') {
            // Add tag
            let tag = tag_part[1..].to_lowercase();
            if !tag.is_empty() && !tags.contains(&tag) {
                tags.push(tag);
            }
        } else if tag_part.starts_with('-') {
            // Remove tag
            let tag = tag_part[1..].to_lowercase();
            tags.retain(|t| t != &tag);
        }
    }
}
