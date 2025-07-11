# Copilot Instructions for Guidebook TODO

## Project Overview

Guidebook TODO is a terminal-based TODO application written in Rust, designed as part of the Guidebook productivity suite. It serves as a companion to `guidebook-plan` and emphasizes fast, efficient task management with rich metadata support.

## Architecture Principles

### Consistency with guidebook-plan

**CRITICAL**: The architecture and code patterns should mirror those used in `guidebook-plan`. The two projects will eventually share a common crate for symmetric behavior, so maintaining consistency is essential.

-   Follow the same project structure and module organization as guidebook-plan
-   Use identical patterns for CLI argument parsing, command handling, and error management
-   Mirror the GitHub OAuth implementation and repository creation logic
-   Maintain consistent coding style, naming conventions, and documentation patterns
-   Use the same approach for data directory management and git integration

### Shared Architecture Goals

-   **Modular Design**: Organize code into clear, reusable modules that can be extracted into a shared crate
-   **Consistent APIs**: Use similar function signatures and error handling patterns across both projects
-   **Common Utilities**: Design utility functions that can be shared between guidebook-plan and guidebook-todo
-   **Unified Configuration**: Follow the same configuration and data management patterns

## Code Organization

### Project Structure

Follow this exact structure to maintain consistency with guidebook-plan:

```
src/
├── main.rs              # Entry point, CLI setup (minimal, delegates to lib)
├── lib.rs               # Library exports and public API
├── cli/
│   ├── mod.rs          # CLI module exports
│   ├── commands.rs     # Command implementations
│   ├── args.rs         # Clap argument definitions
│   └── init.rs         # GitHub OAuth and repo creation (mirror guidebook-plan)
├── core/
│   ├── mod.rs          # Core business logic exports
│   ├── todo.rs         # TODO struct, methods, and validation
│   ├── storage.rs      # File I/O, YAML serialization
│   └── filters.rs      # Search, filter, and sorting logic
├── display/
│   ├── mod.rs          # Display formatting exports
│   ├── table.rs        # List view formatting
│   └── detail.rs       # Detailed view formatting
├── tui/
│   ├── mod.rs          # TUI exports
│   ├── app.rs          # TUI application state management
│   ├── add_todo.rs     # Interactive TODO creation interface
│   ├── search.rs       # Interactive search interface
│   ├── edit_todo.rs    # TODO editing interface
│   ├── components/     # Reusable TUI components
│   │   ├── mod.rs
│   │   ├── input.rs    # Text input widget
│   │   ├── select.rs   # Selection widget
│   │   ├── list.rs     # Search results list widget
│   │   └── textarea.rs # Multi-line text widget
│   └── events.rs       # Event handling and keyboard input
└── config/
    ├── mod.rs          # Configuration exports
    └── defaults.rs     # Fixed application defaults
```

### Module Responsibilities

-   **`cli/`**: Command-line interface, argument parsing, command dispatch
-   **`core/`**: Business logic, data models, core operations
-   **`display/`**: Output formatting, table generation, text rendering
-   **`tui/`**: Terminal user interface components and interactions
-   **`config/`**: Application configuration and default values

## Implementation Guidelines

### Data Models

```rust
// Follow this exact pattern for consistency
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    P0, // Urgent - work on this right now
    P1, // Must have
    P2, // Should do
    P3, // Nice to have
    P4, // Wishlist
    P5, // Worth considering
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Status {
    Todo,
    InProgress,
    Done,
    Cancelled,
    Archived,  // Hidden from default views, preserved for history
}
```

### Error Handling

Use `anyhow` for error handling, following the same patterns as guidebook-plan:

```rust
use anyhow::{Context, Result};

pub fn load_todos() -> Result<TodoList> {
    let path = find_todo_file()
        .context("Failed to locate TODO file")?;

    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read TODO file: {}", path.display()))?;

    serde_yaml::from_str(&content)
        .context("Failed to parse TODO file")
}
```

### CLI Patterns

Mirror guidebook-plan's CLI structure exactly:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A terminal-based TODO application")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        title: Option<String>,
        #[arg(long)]
        quick: bool,
    },
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        priority: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long)]
        all: bool,
    },
    // ... other commands
}
```

### GitHub Integration

**CRITICAL**: Copy the exact OAuth Device Flow implementation from guidebook-plan. This ensures consistency and prepares for code sharing.

Key requirements:

-   Use identical OAuth flow and error handling
-   Mirror the repository creation logic
-   Follow the same user interaction patterns
-   Use consistent terminology and messaging

### Data Directory Management

Follow guidebook-plan's data directory patterns exactly:

```rust
use dirs::home_dir;
use std::path::PathBuf;

pub fn get_data_dir() -> Result<PathBuf> {
    let home = home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    Ok(home.join(".local").join("share").join("guidebook"))
}

pub fn find_todo_file() -> Result<PathBuf> {
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
```

## TUI Development

### Framework Choice

Use `ratatui` with `crossterm` for terminal manipulation, following these principles:

-   **Keyboard-first navigation**: All functionality accessible via keyboard
-   **Responsive design**: Adapt to different terminal sizes
-   **Consistent theming**: Match terminal color preferences
-   **Clear focus indicators**: Always show what element is active

### TUI Component Architecture

```rust
pub trait Component {
    fn draw(&mut self, f: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: &Event) -> bool;
    fn is_focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
}

pub struct TodoEditor {
    title_input: Input,
    priority_select: Select<Priority>,
    status_select: Select<Status>,
    category_input: Input,
    tags_input: Input,
    notes_textarea: TextArea,
    focused_field: usize,
}
```

### Event Handling

Implement consistent event handling patterns:

```rust
use crossterm::event::{Event, KeyCode, KeyModifiers};

pub fn handle_key_event(&mut self, key: KeyEvent) -> bool {
    match (key.modifiers, key.code) {
        (KeyModifiers::CONTROL, KeyCode::Char('x')) | (KeyModifiers::NONE, KeyCode::Esc) => {
            // Auto-save and exit
            self.save_todo();
            true
        },
        (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
            // Save and continue editing
            self.save_todo();
            false
        },
        (KeyModifiers::CONTROL, KeyCode::Char('r')) => {
            // Quick archive shortcut
            self.set_status_archived();
            false
        },
        (KeyModifiers::NONE, KeyCode::Tab) => {
            // Navigate to next field
            self.focus_next_field();
            false
        },
        // ... other key combinations
        _ => false,
    }
}
```

## Testing Strategy

### Unit Tests

Write comprehensive unit tests for all core functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::tempdir;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new("Test task".to_string());
        assert_eq!(todo.title, "Test task");
        assert_eq!(todo.priority, Priority::P2);
        assert_eq!(todo.status, Status::Todo);
    }

    #[test]
    fn test_todo_serialization() {
        let todo = Todo::new("Test task".to_string());
        let yaml = serde_yaml::to_string(&todo).unwrap();
        let deserialized: Todo = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(todo.title, deserialized.title);
    }
}
```

### Integration Tests

Test CLI commands and file operations:

```rust
#[test]
fn test_add_todo_command() {
    let temp_dir = tempdir().unwrap();
    let todo_file = temp_dir.path().join("todo.yaml");

    // Test adding a TODO
    let result = add_todo("Test task", Priority::P1, None, Vec::new());
    assert!(result.is_ok());

    // Verify file was created and contains expected data
    assert!(todo_file.exists());
    let content = std::fs::read_to_string(&todo_file).unwrap();
    assert!(content.contains("Test task"));
}
```

## Performance Requirements

### Startup Time

-   Target: < 50ms for basic operations
-   Lazy load TODO data only when needed
-   Minimize dependency initialization overhead

### Memory Usage

-   Keep memory footprint minimal
-   Stream large TODO lists rather than loading everything
-   Use efficient data structures for filtering and searching

### File I/O

-   Minimize file system operations
-   Cache TODO data in memory during interactive sessions
-   Batch write operations when possible

## Security Considerations

### GitHub OAuth

-   Never store access tokens in plain text
-   Use secure token storage mechanisms
-   Implement proper token refresh flows
-   Follow OAuth security best practices

### File Permissions

-   Set appropriate permissions on TODO files (600)
-   Validate file paths to prevent directory traversal
-   Handle symlinks securely

## Code Quality Standards

### Rust Best Practices

-   Use `clippy` with strict settings
-   Follow Rust naming conventions exactly
-   Prefer `&str` over `String` for function parameters
-   Use `Result<T, E>` for all fallible operations
-   Document all public APIs with rustdoc

### Code Style

```rust
// Function naming: snake_case
pub fn create_todo(title: &str, priority: Priority) -> Result<Todo> {
    // Implementation
}

// Struct naming: PascalCase
pub struct TodoList {
    pub todos: Vec<Todo>,
    pub next_id: u32,
}

// Constant naming: SCREAMING_SNAKE_CASE
const DEFAULT_PRIORITY: Priority = Priority::P2;
const MAX_TITLE_LENGTH: usize = 200;
```

### Documentation

-   Document all public functions and structs
-   Include usage examples in documentation
-   Maintain up-to-date README with examples
-   Use consistent terminology throughout

## Future-Proofing

### Shared Crate Preparation

Structure code to facilitate extraction into a shared crate:

```rust
// Future shared crate structure
guidebook-common/
├── src/
│   ├── lib.rs
│   ├── auth/           # GitHub OAuth (shared)
│   ├── data/           # Data directory management (shared)
│   ├── git/            # Git operations (shared)
│   └── config/         # Configuration patterns (shared)
```

### API Design

Design APIs that can be shared between guidebook-plan and guidebook-todo:

```rust
// Shared authentication trait
pub trait GitHubAuth {
    async fn authenticate(&self) -> Result<String>;
    async fn create_repository(&self, name: &str) -> Result<Repository>;
}

// Shared data directory management
pub trait DataDirectory {
    fn get_data_dir(&self) -> Result<PathBuf>;
    fn ensure_directory_exists(&self, path: &PathBuf) -> Result<()>;
    fn find_project_file(&self, filenames: &[&str]) -> Option<PathBuf>;
}
```

## Development Workflow

### Branch Strategy

-   Use feature branches for new functionality
-   Maintain consistency with guidebook-plan branch naming
-   Create pull requests for all changes

### Testing Requirements

-   All new code must have unit tests
-   Integration tests for CLI commands
-   TUI components should have interaction tests
-   Performance benchmarks for critical paths

### Code Review

-   Focus on consistency with guidebook-plan patterns
-   Verify error handling follows established patterns
-   Ensure documentation is complete and accurate
-   Check for opportunities to extract shared code

## Dependencies Management

### Required Dependencies

Mirror guidebook-plan's dependency choices where possible:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"
colored = "2.0"
anyhow = "1.0"
ratatui = "0.24"
crossterm = "0.27"
reqwest = { version = "0.11", features = ["json"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Version Compatibility

-   Keep dependency versions in sync with guidebook-plan
-   Document any version differences and rationale
-   Plan for shared dependency management in the common crate

## Conclusion

The key to successful development is maintaining strict consistency with guidebook-plan while building toward a shared architecture. Every design decision should consider how it will work in the context of both applications sharing common code.

When in doubt, follow guidebook-plan's patterns exactly. This will make the eventual code sharing much smoother and ensure users have a consistent experience across the Guidebook suite.

### Archive Functionality

The Archived status provides a way to hide TODOs from default views while preserving them for history:

-   **Default Behavior**: Archived TODOs are excluded from all default views (list, search, overview)
-   **Explicit Access**: Use `--all` flag to include archived TODOs in CLI commands
-   **Quick Archive**: Ctrl+R in Edit UI provides instant archiving
-   **Auto-Save**: Edit UI automatically saves changes on exit (Ctrl+X or Esc)

### Filtering Behavior

```rust
// Exclude archived TODOs by default
pub fn get_active_todos<'a>(todos: &'a [Todo]) -> Vec<&'a Todo> {
    todos.iter()
        .filter(|todo| todo.status != Status::Archived)
        .collect()
}

// Include all TODOs when explicitly requested
pub fn get_all_todos_including_archived<'a>(todos: &'a [Todo]) -> Vec<&'a Todo> {
    todos.iter().collect()
}
```
