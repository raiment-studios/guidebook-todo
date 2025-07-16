use crate::core::{Priority, Status, Todo, TodoList};
use anyhow::{Context, Result};
use dirs::home_dir;
use std::path::PathBuf;

pub async fn find_todo_file() -> Result<PathBuf> {
    // 1. Check current directory for project-specific TODOs
    let current_dir = std::env::current_dir()?;
    let local_files = ["TODO.yaml", "TODO.yml", "todo.yaml", "todo.yml"];

    for filename in &local_files {
        let path = current_dir.join(filename);
        if path.exists() {
            return Ok(path);
        }
    }

    // 2. Fall back to global guidebook data directory
    let data_dir = get_data_dir()?;
    let todo_path = data_dir.join("guidebook-todo").join("todo.yaml");

    // Create directory if it doesn't exist
    if let Some(parent) = todo_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Ok(todo_path)
}

pub fn get_data_dir() -> Result<PathBuf> {
    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    Ok(home.join(".local").join("share").join("guidebook"))
}

pub async fn load_todos() -> Result<TodoList> {
    let path = find_todo_file().await?;

    if !path.exists() {
        // Create empty TODO list if file doesn't exist
        let todo_list = TodoList::default();
        save_todos(&todo_list, &path).await?;
        return Ok(todo_list);
    }

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read TODO file: {}", path.display()))?;

    if content.trim().is_empty() {
        return Ok(TodoList::default());
    }

    serde_yaml::from_str(&content).context("Failed to parse TODO file")
}

pub async fn save_todos(todo_list: &TodoList, path: &PathBuf) -> Result<()> {
    let content = serde_yaml::to_string(todo_list).context("Failed to serialize TODO list")?;

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(path, content)
        .with_context(|| format!("Failed to write TODO file: {}", path.display()))?;

    Ok(())
}

impl TodoList {
    pub async fn save(&self) -> Result<()> {
        let path = find_todo_file().await?;
        save_todos(self, &path).await
    }

    pub fn filter_todos(
        &self,
        status: Option<String>,
        category: Option<String>,
        priority: Option<String>,
        tags: Option<String>,
        all: bool,
    ) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| {
                // Exclude archived TODOs unless 'all' flag is set
                if !all && todo.status == crate::core::Status::Archived {
                    return false;
                }

                // Filter by status
                if let Some(ref status_str) = status {
                    if let Ok(target_status) = crate::core::parse_status(status_str) {
                        if todo.status != target_status {
                            return false;
                        }
                    }
                }

                // Filter by category
                if let Some(ref category_str) = category {
                    match &todo.category {
                        Some(cat) if cat == category_str => {}
                        _ => return false,
                    }
                }

                // Filter by priority
                if let Some(ref priority_str) = priority {
                    if let Ok(target_priority) = crate::core::parse_priority(priority_str) {
                        if todo.priority != target_priority {
                            return false;
                        }
                    }
                }

                // Filter by tags
                if let Some(ref tags_str) = tags {
                    let target_tags: Vec<&str> = tags_str.split(',').map(|s| s.trim()).collect();

                    for target_tag in target_tags {
                        if !todo.tags.iter().any(|tag| tag == target_tag) {
                            return false;
                        }
                    }
                }

                true
            })
            .collect()
    }
}

pub async fn default_display() -> Result<()> {
    let todo_list = load_todos().await?;
    let active_todos: Vec<&Todo> = todo_list
        .todos
        .iter()
        .filter(|todo| todo.is_active())
        .collect();

    if active_todos.is_empty() {
        println!("┌─ Your TODOs ────────────────────────────────────────────────────┐");
        println!("│                                                                 │");
        println!("│ No active TODOs found!                                          │");
        println!("│                                                                 │");
        println!("│ Use 'todo add' to create your first TODO                       │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return Ok(());
    }

    // Sort by priority (P0 first) then by creation date (newest first)
    let mut priority_todos = active_todos.clone();
    priority_todos.sort_by(|a, b| match a.priority_value().cmp(&b.priority_value()) {
        std::cmp::Ordering::Equal => b.created_date.cmp(&a.created_date),
        other => other,
    });

    // Take top 4 priority tasks
    let top_priority: Vec<&Todo> = priority_todos.into_iter().take(4).collect();

    // Get 3 random tasks (excluding those already shown)
    let mut remaining_todos: Vec<&Todo> = active_todos
        .into_iter()
        .filter(|todo| !top_priority.iter().any(|t| t.id == todo.id))
        .collect();

    // Simple pseudo-random selection (using ID as seed)
    if !remaining_todos.is_empty() {
        remaining_todos.sort_by_key(|todo| todo.id);
        let step = if remaining_todos.len() > 3 {
            remaining_todos.len() / 3
        } else {
            1
        };
        let random_todos: Vec<&Todo> = remaining_todos.into_iter().step_by(step).take(3).collect();

        display_overview(&top_priority, &random_todos, &todo_list);
    } else {
        display_overview(&top_priority, &[], &todo_list);
    }

    Ok(())
}

fn display_overview(priority_todos: &[&Todo], random_todos: &[&Todo], todo_list: &TodoList) {
    let total_active = todo_list.todos.iter().filter(|t| t.is_active()).count();
    let total_done = todo_list
        .todos
        .iter()
        .filter(|t| matches!(t.status, Status::Done))
        .count();
    let total_in_progress = todo_list
        .todos
        .iter()
        .filter(|t| matches!(t.status, Status::InProgress))
        .count();

    println!("┌─ Your TODOs ────────────────────────────────────────────────────┐");
    println!("│                                                                 │");

    if !priority_todos.is_empty() {
        println!(
            "│ Priority Tasks ({}):                                             │",
            priority_todos.len()
        );
        for todo in priority_todos {
            let icon = get_status_icon(todo);
            let category = todo.category.as_deref().unwrap_or("general");
            let title = if todo.title.len() > 30 {
                format!("{}...", &todo.title[..27])
            } else {
                todo.title.clone()
            };
            println!(
                "│   [{:03}] {:2} | {:8} | {:<30} {} │",
                todo.id, todo.priority, category, title, icon
            );
        }
        println!("│                                                                 │");
    }

    if !random_todos.is_empty() {
        println!(
            "│ Other Tasks ({}):                                                │",
            random_todos.len()
        );
        for todo in random_todos {
            let icon = get_status_icon(todo);
            let category = todo.category.as_deref().unwrap_or("general");
            let title = if todo.title.len() > 30 {
                format!("{}...", &todo.title[..27])
            } else {
                todo.title.clone()
            };
            println!(
                "│   [{:03}] {:2} | {:8} | {:<30} {} │",
                todo.id, todo.priority, category, title, icon
            );
        }
        println!("│                                                                 │");
    }

    println!(
        "│ {} total active • {} completed • {} in progress                 │",
        total_active, total_done, total_in_progress
    );
    println!("│                                                                 │");
    println!("│ Use 'todo search' to find tasks, 'todo add' to create new ones │");
    println!("└─────────────────────────────────────────────────────────────────┘");
}

fn get_status_icon(todo: &Todo) -> &'static str {
    match (&todo.priority, &todo.status) {
        (Priority::P0, _) => "!",
        (Priority::P1, _) => "*",
        (_, Status::InProgress) => ">",
        _ if todo.notes.is_some() => "N",
        _ => " ",
    }
}
