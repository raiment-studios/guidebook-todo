# Guidebook TODO - Terminal-Based TODO Application

## Overview

Guidebook TODO is a fast, efficient, terminal-based TODO application written in Rust and part of the Guidebook suite of productivity tools. It serves as a companion to `guidebook-plan`, providing quick and easy task management with rich metadata support. TODOs are stored in local YAML files with git-based version control for history and restore capabilities.

## Core Features

### Task Properties

The TODO data model is designed for single-user, personal task management:

```rust
pub struct Todo {
    /// Auto-generated unique identifier
    pub id: u32,

    /// Brief description of the task (required, max 200 chars)
    pub title: String,

    /// Task importance level (default: P2)
    pub priority: Priority,

    /// Current task state (default: Todo)
    pub status: Status,

    /// Searchable classification tags (lowercase, no spaces)
    #[serde(default)]
    pub tags: Vec<String>,

    /// Single organizational category (max 50 chars)
    pub category: Option<String>,

    /// Project or context this TODO belongs to (max 100 chars)
    pub project: Option<String>,

    /// Auto-generated creation timestamp (immutable)
    pub created_date: DateTime<Local>,

    /// Auto-populated when status changes to Done
    pub finished_date: Option<DateTime<Local>>,

    /// Multi-line additional details (max 2000 chars)
    pub notes: Option<String>,
}

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

pub enum Status {
    Todo,
    InProgress,
    Done,
    Archived,    // Hidden from default views, preserved for history
}


/// Container for all TODOs in the YAML file
pub struct TodoList {
    /// Next ID to assign to new TODOs
    pub next_id: u32,

    #[serde(default)]
    pub todos: Vec<Todo>,
}
```

### Data Storage

-   **Format**: YAML file for human readability and easy editing
-   **Location Priority** (checks in this order):
    1. Current working directory: `TODO.yaml`, `TODO.yml`, `todo.yaml`, or `todo.yml`
    2. User data directory: `~/.local/share/guidebook/guidebook-todo/todo.yaml`
-   **Version Control**: Relies on git for history and restore capabilities (no backup files needed)
-   **Single-User**: Designed for personal task management, no multi-user considerations
-   **Guidebook Integration**: Part of the Guidebook productivity suite

### Data File Discovery

The application uses the following logic to locate the TODO data file:

```rust
use std::path::{Path, PathBuf};

fn find_todo_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Check current working directory first
    let current_dir = std::env::current_dir()?;
    let local_files = ["TODO.yaml", "TODO.yml", "todo.yaml", "todo.yml"];

    for filename in &local_files {
        let path = current_dir.join(filename);
        if path.exists() {
            return Ok(path);
        }
    }    // Fall back to user data directory
    let home_dir = dirs::home_dir()
        .ok_or("Could not determine home directory")?;
    let data_path = home_dir
        .join(".local")
        .join("share")
        .join("guidebook")
        .join("guidebook-todo")
        .join("todo.yaml");

    // Create directory if it doesn't exist
    if let Some(parent) = data_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Ok(data_path)
}
```

This allows users to:

-   Keep project-specific TODO lists in their project directories
-   Have a personal global TODO list in the Guidebook data directory (git-managed)
-   Use any of the common YAML file extensions
-   Leverage git for version control and history tracking

## Command Line Interface

### Basic Commands

#### Initialize guidebook-todo

The `todo init` command sets up guidebook-todo for first-time use by creating the necessary directory structure and optionally a GitHub repository:

```bash
todo init                           # Interactive setup for first-time users
```

**Initialization Process**:

1. **Check existing setup**: Verifies if `~/.local/share/guidebook/` already exists
2. **Directory discovery**: Checks for local TODO files in current directory
3. **GitHub integration**: Offers to create a remote repository for data backup/sync
4. **Default structure**: Creates initial directory structure and sample TODO file

**Interactive Setup Flow**:

```
guidebook-todo could not find an existing data directory.

guidebook-todo stores its data locally with optional GitHub backup.
If you already have a 'guidebook-data' repository on GitHub, you can
link it for automatic synchronization.

Would you like to:

1. Create a new guidebook-data repository on GitHub
2. Skip GitHub integration (local-only mode)
3. Exit without making changes

Enter your choice (1-3): _
```

**GitHub Authentication**: Uses GitHub OAuth Device Flow for secure, user-friendly authentication without requiring tokens or passwords.

#### Add a new TODO

The `todo add` command opens an interactive TUI editor for creating new TODOs:

```bash
todo add                            # Opens interactive TUI editor
todo add "Quick task"               # Pre-fills title and opens editor
todo add --quick "Simple task"      # Bypass TUI, create with defaults
```

**Interactive TUI Editor**:

```
┌─────────────────────────────────────────────────────────────────┐
│ Add New TODO                                              [Esc] │
├─────────────────────────────────────────────────────────────────┤
│ Title: Fix the login bug_                                       │
│                                                                 │
│ Priority: [P0] P1  P2  P3  P4  P5             [Tab/Arrow Keys]  │
│ Status:   [Todo] InProgress                   [Tab/Arrow Keys]  │
│ Category: work_                               [Type to edit]    │
│ Tags:     bug, urgent_                        [Comma separated] │
│                                                                 │
│ Notes:                                        [Ctrl+N to edit]  │
│ ┌─────────────────────────────────────────────────────────────┐ │
│ │ Users are reporting authentication failures after the       │ │
│ │ latest update. Need to investigate JWT token validation     │ │
│ │ logic._                                                     │ │
│ │                                                             │ │
│ └─────────────────────────────────────────────────────────────┘ │
│                                                                 │
│ [Ctrl+R] Archive  [Ctrl+S] Save  [Ctrl+X] Exit (auto-saves)    │
└─────────────────────────────────────────────────────────────────┘
```

**Keyboard Controls**:

-   `Tab` / `Shift+Tab`: Navigate between fields
-   `Arrow Keys`: Navigate within multi-option fields (Priority, Status)
-   `Enter`: Confirm selection in multi-option fields
-   `Ctrl+R`: Quick archive (sets status to Archived)
-   `Ctrl+S`: Save and continue editing
-   `Ctrl+X` / `Esc`: Auto-save and exit
-   `F1`: Show help overlay

#### List TODOs

```bash
todo list                           # Show all active todos (excludes archived)
todo list --status done             # Show completed todos
todo list --status archived         # Show archived todos
todo list --category work           # Filter by category
todo list --priority p0             # Filter by priority
todo list --tags urgent             # Filter by tags
todo list --all                     # Show all todos including archived
```

#### Update a TODO

```bash
todo update 123 --status done       # Mark as complete
todo update 123 --status archived   # Archive the todo
todo update 123 --priority p3       # Change priority
todo update 123 --tags +bug,-urgent # Add 'bug' tag, remove 'urgent' tag
todo update 123 --notes "Additional details here"
```

#### Delete a TODO

```bash
todo delete 123                     # Delete by ID
todo delete --category temp         # Delete all in category
todo delete --status archived      # Delete all archived items
```

#### Search TODOs

The `todo search` command opens an interactive TUI search interface:

```bash
todo search                         # Opens interactive TUI search
todo search "login"                 # Pre-fills search term and opens TUI
```

**Interactive TUI Search Interface**:

```
┌─────────────────────────────────────────────────────────────────┐
│ Search TODOs                                              [Esc] │
├─────────────────────────────────────────────────────────────────┤
│ Search: login_                                  [Type to filter] │
├─────────────────────────────────────────────────────────────────┤
│ [001] P0       | work     | Fix the login bug              !   │
│ [015] P2       | backend  | Update login validation logic       │
│ [023] P4       | docs     | Document login flow                 │
│                                                                 │
│ 3 results found                                                 │
│                                                                 │
│ [Enter] Edit Selected  [↑↓] Navigate  [Ctrl+C] Cancel          │
│ [/] Focus Search  [Tab] Cycle Filters  [F1] Help               │
└─────────────────────────────────────────────────────────────────┘
```

**Search Features**:

-   **Real-time filtering**: Results update as you type
-   **Multi-field search**: Searches title, notes, tags, and category
-   **Filter toggles**: Quick filters for status, priority, category
-   **Keyboard navigation**: Arrow keys to select results
-   **Direct editing**: Enter to edit selected TODO in TUI editor
-   **Seamless workflow**: Returns to search results after editing

**Keyboard Controls**:

-   `Type`: Filter results in real-time
-   `↑↓` / `Arrow Keys`: Navigate through results
-   `Enter`: Edit selected TODO (opens TUI editor)
-   `/`: Focus search input field
-   `Tab`: Cycle through quick filters (status, priority, category)
-   `Ctrl+C` / `Esc`: Exit search interface
-   `F1`: Show help overlay

#### Interactive Search & Edit Workflow

The search command provides a seamless workflow for finding and editing TODOs:

1. **Search Phase**: Type to filter TODOs in real-time
2. **Selection Phase**: Use arrow keys to highlight desired TODO
3. **Edit Phase**: Press Enter to open TODO in TUI editor
4. **Return Phase**: After saving/canceling edit, returns to search results

**Advanced Search Features**:

-   **Fuzzy matching**: Finds partial matches across all text fields
-   **Tag filtering**: Type `#tag` to filter by specific tags
-   **Category filtering**: Type `@category` to filter by category
-   **Status filtering**: Type `!status` to filter by status
-   **Priority filtering**: Type `p0` for urgent items, `p1` for must-have, etc.

**Search Result Display**:

```
[ID] Priority | Category | Title                    | Status Icons
[001] P0      | work     | Fix the login bug        | !*
[015] P2      | backend  | Update validation logic  | >
[023] P4      | docs     | Document login flow      | N
```

**Status Icons**:

-   ! P0 (Urgent)
-   -   P1 (Must have)
-   > In Progress
-   -   Done
-   A Archived
-   N Has notes

**Interactive Controls**:

-   **↑↓** Navigate between results
-   **⏎** Edit selected TODO
-   **/** Focus search input
-   **+/=** Increase priority (make it higher priority)
-   **-** Decrease priority (make it lower priority)
-   **F1** Toggle help
-   **⌃X** Exit

**Visual Enhancements**:

-   **ID numbers** are displayed in a dimmer color for reduced visual noise
-   **Priority values** are color-coded using the Apollo palette:
    -   P0 (Urgent): Bright Magenta
    -   P1 (High): Bright Orange
    -   P2 (Medium): Yellow
    -   P3-P4 (Low): Lime Green
    -   P5 (Wishlist): Light Cyan
-   **Selected item** is highlighted with background color and bold text

### Advanced Commands

#### Show TODO details

```bash
todo show 123                       # Show full details of a specific TODO
```

#### Statistics

```bash
todo stats                          # Show completion stats, category breakdown
```

## Default Behavior (No Command)

When running `todo` without any commands, the application displays a quick overview of tasks to help users stay focused:

```bash
todo                                # Display overview of current tasks
```

**Default Display Logic**:

1. **Top Priority Tasks** (4 items): Shows the 4 highest-priority active TODOs, sorted by:
    - Priority level (P0 → P5)
    - Creation date (newest first) as tiebreaker
2. **Random Selection** (3 items): Shows 3 randomly selected active TODOs (excluding those already shown)
3. **Active Filter**: Only includes TODOs with status `Todo` or `InProgress`

**Sample Default Output**:

```
┌─ Your TODOs ────────────────────────────────────────────────────┐
│                                                                 │
│ Priority Tasks (4):                                             │
│   [001] P0 | work     | Fix critical login bug           !*   │
│   [007] P0 | personal | Submit tax documents             !    │
│   [015] P1 | backend  | Update API validation logic     *    │
│   [023] P1 | home     | Schedule HVAC maintenance        *    │
│                                                                 │
│ Other Tasks (3):                                                │
│   [034] P2 | docs     | Update README examples           >    │
│   [041] P3 | personal | Plan weekend hiking trip              │
│   [052] P4 | home     | Research new coffee makers           │
│                                                                 │
│ 47 total active • 12 completed • 3 in progress                 │
│                                                                 │
│ Use 'todo search' to find tasks, 'todo add' to create new ones │
└─────────────────────────────────────────────────────────────────┘
```

**Status Icons**:

-   `!` P0 (Urgent)
-   `*` P1 (Must have)
-   `>` In Progress
-   `N` Has notes

**Edge Cases**:

-   If fewer than 4 high-priority tasks exist, shows all available
-   If fewer than 7 total active tasks, skips random selection
-   Empty TODO list shows helpful getting-started message

## Display Formatting

### List View

```
ID   | Priority | Status     | Category | Title                    | Tags
-----|----------|------------|----------|--------------------------|----------
001  | P0       | Todo       | work     | Fix the login bug        | bug,urgent
002  | P2       | InProgress | personal | Plan vacation            | travel
003  | P4       | Done       | home     | Fix leaky faucet         |
```

### Detailed View

```
TODO #001
Title: Fix the login bug
Status: Todo
Priority: P0
Category: work
Tags: bug, urgent
Created: 2025-07-11 14:30:00
Finished: -
Notes:
  Users are reporting authentication failures after the latest update.
  Need to investigate JWT token validation logic.
```

## Application Configuration

The application uses fixed configuration for simplicity - no user configuration files are needed. Default behaviors include:

-   **Display**: Show IDs, use colors, format dates as "YYYY-MM-DD HH:MM"
-   **Filters**: Hide Done/Archived TODOs by default in list view
-   **Data**: Prefer local TODO files in current directory
-   **TUI**: Default theme with help footer enabled
-   **Priority**: Default new TODOs to P2 (Should do)

## Technical Requirements

### Dependencies

-   **clap**: Command-line argument parsing
-   **serde**: Serialization/deserialization
-   **serde_yaml**: YAML support
-   **chrono**: Date/time handling
-   **dirs**: XDG directory support
-   **colored**: Terminal color output
-   **anyhow**: Error handling
-   **ratatui**: Terminal User Interface framework
-   **crossterm**: Cross-platform terminal manipulation
-   **reqwest**: HTTP client for GitHub API integration
-   **serde_json**: JSON handling for GitHub API responses
-   **tokio**: Async runtime for GitHub OAuth flow

### Project Structure

```
src/
├── main.rs              # Entry point and CLI setup
├── lib.rs               # Library exports
├── cli/
│   ├── mod.rs
│   ├── commands.rs      # Command implementations
│   ├── args.rs          # Argument definitions
│   └── init.rs          # GitHub OAuth and repository creation
├── core/
│   ├── mod.rs
│   ├── todo.rs          # TODO struct and methods
│   ├── storage.rs       # File I/O operations
│   └── filters.rs       # Search and filter logic
├── display/
│   ├── mod.rs
│   ├── table.rs         # List view formatting
│   └── detail.rs        # Detailed view formatting
├── tui/
│   ├── mod.rs
│   ├── app.rs           # TUI application state
│   ├── add_todo.rs      # Interactive TODO creation
│   ├── search.rs        # Interactive search interface
│   ├── edit_todo.rs     # TODO editing interface
│   ├── components/      # Reusable TUI components
│   │   ├── mod.rs
│   │   ├── input.rs     # Text input widget
│   │   ├── select.rs    # Selection widget
│   │   ├── list.rs      # Search results list widget
│   │   └── textarea.rs  # Multi-line text widget
│   └── events.rs        # Event handling
└── config/
    ├── mod.rs
    └── defaults.rs      # Fixed application defaults (no user config)
```

### Data Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub tags: Vec<String>,
    pub category: Option<String>,
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
    Archived,
}
```

## Error Handling

-   Graceful handling of file I/O errors
-   Clear error messages for invalid commands
-   Validation of TODO IDs and parameters
-   Automatic creation of Guidebook directories if missing
-   Git-aware error handling for version control operations

## Performance Considerations

-   Lazy loading of TODO data
-   Efficient filtering and searching
-   Minimal memory footprint
-   Fast startup time (< 50ms for basic operations)

## Future Enhancements

### Phase 2 Features

-   Due dates and reminders
-   Recurring tasks
-   Time tracking
-   Multiple TODO lists/projects
-   Integration with `guidebook-plan` for shared task management
-   Full TUI mode for browsing and managing TODOs
-   Export functionality for data portability
-   Advanced GitHub integration (branch-aware TODOs, conflict resolution)

### Phase 3 Features

-   Cross-Guidebook tool synchronization
-   Advanced git integration (branch-aware TODOs)
-   Plugin system compatible with Guidebook suite
-   Integration with calendar applications
-   Advanced TUI features (themes, custom layouts)
-   Vim-style key bindings and navigation

## Installation

### From Source

```bash
git clone https://github.com/user/guidebook-todo
cd guidebook-todo
cargo build --release
cargo install --path .
```

### From Crates.io

```bash
cargo install guidebook-todo
```

## Testing Strategy

-   Unit tests for core TODO operations
-   Integration tests for CLI commands
-   Property-based testing for data integrity
-   Performance benchmarks for large TODO lists
-   Cross-platform compatibility testing

## Documentation

-   Comprehensive README with examples
-   Man page generation
-   Built-in help system (`todo help`)
-   Online documentation with tutorials

## Terminal User Interface (TUI)

### Design Principles

-   **Keyboard-First**: All functionality accessible via keyboard shortcuts
-   **Intuitive Navigation**: Tab/arrow key navigation between elements
-   **Visual Feedback**: Clear highlighting of active elements and available actions
-   **Responsive Design**: Adapts to different terminal sizes
-   **Consistent Themes**: Color scheme matches terminal preferences

### TUI Components

#### Interactive TODO Creator

The primary TUI interface launched by `todo add`:

**Features**:

-   Real-time input validation
-   Auto-completion for categories and tags based on existing TODOs
-   Contextual help system
-   Preview of how the TODO will appear in list view
-   Smart defaults based on user preferences

**Accessibility**:

-   Screen reader compatible
-   High contrast mode support
-   Keyboard-only navigation
-   Clear focus indicators

#### Smart Auto-completion

-   **Categories**: Suggests existing categories as you type
-   **Tags**: Shows previously used tags with frequency indicators
-   **Text Fields**: Basic text editing with undo/redo support

#### Help System

-   `F1`: Context-sensitive help overlay
-   Shows available keyboard shortcuts for current field
-   Interactive tutorial for first-time users

### UX Enhancements

#### Quick Actions

-   `Ctrl+Enter`: Save with current selections (skip remaining fields)
-   `Ctrl+D`: Use defaults for all remaining fields
-   `Ctrl+R`: Reset form to initial state

#### Visual Indicators

-   Required fields marked with `*`
-   Validation errors shown inline with red highlighting
-   Character count for title field
-   Tag count indicator

#### Field-Specific Features

**Title Field**:

-   Auto-capitalize first letter
-   Real-time character count (recommended: 20-50 chars)
-   Suggests completion based on partial matches

**Priority Field**:

-   Visual indicators: `!` P0, `*` P1, `-` P2, `~` P3, `.` P4, ` ` P5
-   Color coding: Red (P0), Orange (P1), Yellow (P2), Blue (P3), Green (P4), Gray (P5)
-   Descriptions shown on hover/selection

**Category Field**:

-   Dropdown with existing categories
-   Create new category option
-   Shows TODO count per category

**Tags Field**:

-   Tag suggestions appear as you type
-   Visual tag bubbles with easy removal (`Backspace` to remove last tag)
-   Duplicate tag prevention

**Notes Field**:

-   Rich text editing area with word wrap
-   Markdown syntax highlighting (basic)
-   Line numbers for longer notes

## Guidebook Suite Integration

As part of the Guidebook productivity suite, `guidebook-todo` integrates seamlessly with other Guidebook tools:

### Companion to guidebook-plan

-   **Shared ecosystem**: Works alongside `guidebook-plan` for comprehensive project management
-   **Consistent data location**: Uses `~/.local/share/guidebook/` for centralized data storage
-   **Git-based workflow**: Leverages git for version control instead of traditional backup files

### Data Management Philosophy

-   **Git-first approach**: All data is stored in git-trackable formats (YAML)
-   **No backup files needed**: Git provides superior version control and history
-   **Fixed configuration**: No user configuration files - uses sensible defaults
-   **Shared data location**: Follows Guidebook suite conventions
-   **Auto-initialization**: Creates GitHub repository automatically on first run
-   **Cross-device sync**: Seamless synchronization via git operations

### Directory Structure

```
~/.local/share/guidebook/
├── guidebook-plan/          # Plan-related data
├── guidebook-todo/          # TODO application data
│   └── todo.yaml           # Global TODO list (if no local file found)
└── .git/                   # Git repository for version control
```

### Automatic Repository Creation

When a user runs `guidebook-todo` for the first time and no data directory exists:

```rust
use std::path::PathBuf;
use reqwest::Client;
use serde_json::{json, Value};

async fn handle_first_run() -> Result<()> {
    let data_dir = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".local/share/guidebook");

    if !data_dir.exists() {
        println!("Welcome to guidebook-todo!");
        println!("Setting up your TODO management system...");

        // Offer GitHub integration
        if prompt_github_setup()? {
            create_github_repository().await?;
            clone_repository(&data_dir).await?;
        } else {
            // Local-only setup
            std::fs::create_dir_all(&data_dir.join("guidebook-todo"))?;
            create_default_todo_file(&data_dir)?;
        }

        println!("✓ guidebook-todo is ready to use!");
    }

    Ok(())
}

async fn create_github_repository() -> Result<String> {
    // Uses GitHub OAuth Device Flow (same as guidebook-plan)
    let client_id = "your-client-id"; // Replace with actual client ID
    let client = Client::new();

    // 1. Request device code
    let device_response = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[("client_id", client_id), ("scope", "repo")])
        .send()
        .await?;

    let device_data: Value = device_response.json().await?;
    let verification_uri = device_data["verification_uri"].as_str().unwrap();
    let user_code = device_data["user_code"].as_str().unwrap();

    // 2. Show user instructions
    println!("To authorize guidebook-todo, visit: {}", verification_uri);
    println!("And enter the code: {}", user_code);
    println!("Waiting for authorization...");

    // 3. Poll for token (implementation similar to guidebook-plan)
    let access_token = poll_for_token(&client, client_id, &device_data).await?;

    // 4. Create repository
    let repo_response = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", access_token))
        .json(&json!({
            "name": "guidebook-data",
            "description": "Personal guidebook data repository",
            "private": false, // Public for easier sharing/backup
            "auto_init": true
        }))
        .send()
        .await?;

    Ok(access_token)
}
```

**Benefits of Auto-Initialization**:

-   **Zero-friction onboarding**: Works immediately for new users
-   **GitHub integration**: Automatic backup and sync across devices
-   **Graceful fallback**: Works offline or without GitHub if preferred
-   **One-time setup**: Never bothers the user again after initial setup

---

## Color System

Guidebook TODO uses a compile-time configurable color palette system with RGB truecolor support. By default, it uses the Apollo palette from Lospec, created by AdamCYounis.

### Apollo Palette

The Apollo palette provides 32 carefully chosen colors that work well together for terminal interfaces:

-   **Dark blues and cyans**: Good for backgrounds and secondary elements
-   **Greens**: Success states and highlights
-   **Warm browns and oranges**: Text and neutral elements
-   **Purples and magentas**: Special states and warnings
-   **Neutrals**: Various grays from light to dark

### Color Theme Architecture

#### Core Structure

The color system is built around three main components:

1. **`ApolloRgb`**: Raw RGB values for all Apollo palette colors
2. **`ApolloTheme`**: Semantic color assignments (primary, secondary, etc.)
3. **`ColorTheme` trait**: Compile-time configurable interface

#### Semantic Color Categories

**UI Colors**

-   `primary()` - Main accent color (Sky Blue)
-   `secondary()` - Secondary accent (Light Cyan)
-   `accent()` - Highlight color (Yellow)
-   `success()` - Success states (Bright Green)
-   `warning()` - Warning states (Bright Orange)
-   `error()` - Error states (Bright Magenta)

**Text Colors**

-   `text_primary()` - Main text (Light Cream)
-   `text_secondary()` - Secondary text (Light Gray)
-   `text_muted()` - Muted text (Medium Gray)
-   `text_disabled()` - Disabled text (Dark Gray)

**Background Colors**

-   `background()` - Main background (Near Black)
-   `background_alt()` - Alternative background (Midnight)
-   `surface()` - UI surface (Darkest Gray)
-   `surface_alt()` - Alternative surface (Darker Gray)

**Interactive States**

-   `focused()` - Focused elements (Yellow)
-   `selected()` - Selected items (Darker Gray)
-   `hover()` - Hover states (Dark Blue)

**TODO-Specific Colors**

-   `priority_urgent()` - P0 priority (Bright Magenta)
-   `priority_high()` - P1 priority (Bright Orange)
-   `priority_medium()` - P2 priority (Yellow)
-   `priority_low()` - P3-P4 priority (Lime Green)
-   `priority_wishlist()` - P5 priority (Light Cyan)

**Status Colors**

-   `status_todo()` - Todo status (Text Primary)
-   `status_in_progress()` - In Progress (Sky Blue)
-   `status_done()` - Done status (Bright Green)
-   `status_archived()` - Archived (Medium Gray)

### Using the Color System

**In TUI Components**:

```rust
use crate::config::{Theme, ColorTheme};

// Use semantic colors instead of hardcoded ones
let style = if focused {
    Style::default().fg(Theme::focused())
} else {
    Style::default().fg(Theme::text_muted())
};
```

**In Display Logic**:

```rust
let priority_color = match todo.priority {
    Priority::P0 => Theme::priority_urgent(),
    Priority::P1 => Theme::priority_high(),
    Priority::P2 => Theme::priority_medium(),
    // etc.
};
```

### Changing the Color Palette

The color system is designed to be compile-time configurable. To use a different palette:

**Option 1: Modify the Default Theme**

Edit `src/config/colors.rs` and change the type alias:

```rust
// Change from:
pub type Theme = DefaultTheme;

// To your custom theme:
pub type Theme = MyCustomTheme;
```

**Option 2: Create a Custom Theme**

1. Create your own RGB values:

```rust
pub struct MyPaletteRgb;
impl MyPaletteRgb {
    pub const PRIMARY: (u8, u8, u8) = (0x1a, 0x1b, 0x2e);
    // ... more colors
}
```

2. Create a theme struct and implement the `ColorTheme` trait
3. Update the type alias to use your custom theme

### RGB Truecolor Support

The color system uses RGB truecolor (`Color::Rgb(r, g, b)`) which provides:

-   **16.7 million colors** instead of the basic 16 terminal colors
-   **Consistent appearance** across different terminals that support truecolor
-   **Fine-grained color control** for better visual hierarchy

**Terminal Compatibility**: Most modern terminals support truecolor (iTerm2, Terminal.app, Windows Terminal, GNOME Terminal, Konsole, VS Code integrated terminal).

### Benefits of This Architecture

1. **Consistency**: All colors come from a single, cohesive palette
2. **Maintainability**: Easy to change colors application-wide
3. **Flexibility**: Compile-time configuration allows different themes
4. **Semantic**: Color names reflect their purpose, not appearance
5. **Future-proof**: Easy to extend with new color categories

---

## Push Command

The `push` command provides GitHub integration similar to guidebook-plan's push functionality.

### Command Usage

```bash
# Push changes with automatic commit message
todo push

# Push changes with custom commit message
todo push --message "Your custom commit message"

# Force push even when no changes are detected
todo push --force

# Combine custom message with force
todo push --message "Custom message" --force
```

### Features

**🔄 Automatic Change Detection**

-   Detects if there are uncommitted changes in the guidebook data directory
-   Only commits and pushes when changes are present
-   Use `--force` to push even without changes

**📝 Smart Commit Messages**

-   **Custom Message**: Use `--message "Your message"` for specific commit messages
-   **Automatic Message**: Generates timestamp-based messages like "Update TODOs - 2025-07-11 14:17"
-   Maintains a clean commit history

**🔧 Git Integration**

-   Automatically stages all changes (`git add .`)
-   Commits changes with appropriate messages
-   Pushes to `origin main` branch
-   Handles upstream branch setup automatically

**🛡️ Error Handling**

-   Validates guidebook-todo is initialized (`todo init` required first)
-   Checks for git repository presence
-   Provides clear error messages for git operation failures
-   Restores working directory on any errors

### Workflow Integration

The push command fits seamlessly into the guidebook-todo workflow:

1. **Make Changes**: Add, edit, or delete TODOs
2. **Push Updates**: `todo push` to sync with GitHub
3. **Custom Messages**: Use `--message` for significant updates
4. **Force Push**: Use `--force` for clean pushes without changes

### Examples

```bash
# Daily workflow
todo add --quick "Review project proposal"
todo add --quick "Call dentist for appointment"
todo push --message "Add daily tasks"

# Quick sync
todo edit 123  # Make some edits in TUI
todo push      # Auto-commit with timestamp

# Clean push for synchronization
todo push --force --message "Sync repository state"
```

### Implementation Details

**Directory Management**

-   Operates in the guidebook data directory (`~/.local/share/guidebook`)
-   Temporarily changes working directory for git operations
-   Always restores original directory, even on errors

**Git Commands Used**

-   `git status --porcelain` - Check for changes
-   `git add .` - Stage all changes
-   `git commit -m "message"` - Commit with message
-   `git push origin main` - Push to GitHub
-   `git push --set-upstream origin main` - Set upstream if needed

**Error Recovery**

-   Handles "no upstream branch" scenarios automatically
-   Provides meaningful error messages for common git issues
-   Ensures working directory is always restored

**Integration with guidebook-plan**

-   Similar CLI argument structure
-   Consistent error handling and messaging
-   Compatible GitHub repository structure
-   Shared data directory organization

This ensures a unified experience across the Guidebook suite of tools.
