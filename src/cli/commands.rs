use crate::cli::args::{Cli, Commands};
use crate::core::{default_display, find_todo_file, load_todos, TodoList};
use crate::display::{format_detail, format_list};
use crate::tui::{run_add_todo, run_edit_todo, run_search_todo};
use anyhow::{Context, Result};
use dirs::home_dir;
use std::path::PathBuf;
use std::process::Command;

pub async fn run_command(cli: Cli) -> Result<()> {
    match cli.command {
        None => {
            // Default behavior: show overview
            default_display().await?;
        }
        Some(Commands::Init) => {
            crate::cli::init::run_init().await?;
        }
        Some(Commands::Add { title, quick }) => {
            if quick {
                // Quick add with defaults
                quick_add_todo(title).await?;
            } else {
                // Interactive TUI add
                run_add_todo(title).await?;
            }
        }
        Some(Commands::List {
            status,
            category,
            priority,
            tags,
            all,
        }) => {
            list_todos(status, category, priority, tags, all).await?;
        }
        Some(Commands::Update {
            id,
            status,
            priority,
            tags,
            category,
            project,
            notes,
        }) => {
            update_todo(id, status, priority, tags, category, project, notes).await?;
        }
        Some(Commands::Delete {
            id,
            category,
            status,
        }) => {
            delete_todo(id, category, status).await?;
        }
        Some(Commands::Search { query }) => {
            run_search_todo(query).await?;
        }
        Some(Commands::Edit { id }) => {
            run_edit_todo(id).await?;
        }
        Some(Commands::Show { id }) => {
            show_todo(id).await?;
        }
        Some(Commands::Stats) => {
            show_stats().await?;
        }
        Some(Commands::Push { message, force }) => {
            push_to_github(message, force).await?;
        }
        Some(Commands::Code) => {
            open_in_vscode().await?;
        }
    }
    Ok(())
}

async fn quick_add_todo(title: Option<String>) -> Result<()> {
    let title = title.unwrap_or_else(|| {
        println!("Enter TODO title:");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

    if title.is_empty() {
        anyhow::bail!("TODO title cannot be empty");
    }

    let mut todo_list = load_todos().await?;
    let todo = todo_list.create_todo(title);
    todo_list.add_todo(todo);
    todo_list.save().await?;

    println!("✓ TODO added successfully");
    Ok(())
}

async fn list_todos(
    status: Option<String>,
    category: Option<String>,
    priority: Option<String>,
    tags: Option<String>,
    all: bool,
) -> Result<()> {
    let todo_list = load_todos().await?;
    let filtered_todos = todo_list.filter_todos(status, category, priority, tags, all);
    format_list(&filtered_todos);
    Ok(())
}

async fn update_todo(
    id: u32,
    status: Option<String>,
    priority: Option<String>,
    tags: Option<String>,
    category: Option<String>,
    project: Option<String>,
    notes: Option<String>,
) -> Result<()> {
    let mut todo_list = load_todos().await?;
    todo_list.update_todo(id, status, priority, tags, category, project, notes)?;
    todo_list.save().await?;
    println!("✓ TODO updated successfully");
    Ok(())
}

async fn delete_todo(
    id: Option<u32>,
    category: Option<String>,
    status: Option<String>,
) -> Result<()> {
    let mut todo_list = load_todos().await?;
    if let Some(id) = id {
        todo_list.delete_todo(id)?;
        println!("✓ TODO deleted successfully");
    } else if let Some(category) = category {
        let count = todo_list.delete_by_category(&category);
        println!("✓ Deleted {} TODOs in category '{}'", count, category);
    } else if let Some(status) = status {
        let count = todo_list.delete_by_status(&status)?;
        println!("✓ Deleted {} TODOs with status '{}'", count, status);
    } else {
        anyhow::bail!("Must specify either ID, category, or status");
    }
    todo_list.save().await?;
    Ok(())
}

async fn show_todo(id: u32) -> Result<()> {
    let todo_list = load_todos().await?;
    if let Some(todo) = todo_list.get_todo(id) {
        format_detail(todo);
    } else {
        anyhow::bail!("TODO with ID {} not found", id);
    }
    Ok(())
}

async fn show_stats() -> Result<()> {
    let todo_list = load_todos().await?;
    todo_list.show_stats();
    Ok(())
}

async fn push_to_github(message: Option<String>, force: bool) -> Result<()> {
    let data_dir = get_data_dir()?;

    if !data_dir.exists() {
        anyhow::bail!("guidebook-todo is not initialized. Run 'todo init' first.");
    }

    // Check if we're in a git repository
    if !data_dir.join(".git").exists() {
        anyhow::bail!("Not a git repository. Run 'todo init' to set up GitHub integration.");
    }

    // Change to data directory
    let current_dir = std::env::current_dir()?;
    std::env::set_current_dir(&data_dir)?;

    let result = async {
        // Check if there are any changes to commit
        let status_output = Command::new("git")
            .args(&["status", "--porcelain"])
            .output()
            .context("Failed to check git status")?;

        if !status_output.status.success() {
            anyhow::bail!(
                "Git status failed: {}",
                String::from_utf8_lossy(&status_output.stderr)
            );
        }

        let has_changes = !status_output.stdout.is_empty();

        if !has_changes && !force {
            println!("No changes to push. Use --force to push anyway.");
            return Ok(());
        }

        if has_changes {
            // Add all changes
            let add_output = Command::new("git")
                .args(&["add", "."])
                .output()
                .context("Failed to add files to git")?;

            if !add_output.status.success() {
                anyhow::bail!(
                    "Git add failed: {}",
                    String::from_utf8_lossy(&add_output.stderr)
                );
            }

            // Commit changes
            let commit_message = message.unwrap_or_else(|| {
                format!(
                    "Update TODOs - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M")
                )
            });

            let commit_output = Command::new("git")
                .args(&["commit", "-m", &commit_message])
                .output()
                .context("Failed to commit changes")?;

            if !commit_output.status.success() {
                anyhow::bail!(
                    "Git commit failed: {}",
                    String::from_utf8_lossy(&commit_output.stderr)
                );
            }

            println!("✓ Changes committed: {}", commit_message);
        }

        // Push to GitHub
        let push_output = Command::new("git")
            .args(&["push", "origin", "main"])
            .output()
            .context("Failed to push to GitHub")?;

        if !push_output.status.success() {
            let stderr = String::from_utf8_lossy(&push_output.stderr);

            // Check if it's a "no upstream branch" error
            if stderr.contains("no upstream branch") {
                println!("Setting up upstream branch...");

                let upstream_output = Command::new("git")
                    .args(&["push", "--set-upstream", "origin", "main"])
                    .output()
                    .context("Failed to set upstream branch")?;

                if !upstream_output.status.success() {
                    anyhow::bail!(
                        "Failed to set upstream: {}",
                        String::from_utf8_lossy(&upstream_output.stderr)
                    );
                }
            } else {
                anyhow::bail!("Git push failed: {}", stderr);
            }
        }

        println!("✓ Successfully pushed to GitHub!");
        Ok(())
    }
    .await;

    // Always restore the original directory
    std::env::set_current_dir(current_dir)?;
    result
}

fn get_data_dir() -> Result<PathBuf> {
    let home = home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    Ok(home.join(".local").join("share").join("guidebook"))
}

async fn open_in_vscode() -> Result<()> {
    let todo_file_path = find_todo_file().await?;

    // Check if the file exists, create it if it doesn't
    if !todo_file_path.exists() {
        println!("TODO file doesn't exist yet. Creating empty file...");
        if let Some(parent) = todo_file_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Create an empty TODO list file
        let empty_todo_list = TodoList::new();
        empty_todo_list.save().await?;
    }

    // Try to open with VS Code
    println!("Opening TODO file in VS Code: {}", todo_file_path.display());

    let output = Command::new("code")
        .arg(&todo_file_path)
        .output()
        .context("Failed to execute 'code' command. Make sure VS Code is installed and the 'code' command is available in your PATH.")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to open file in VS Code: {}", stderr);
    }

    println!("✓ TODO file opened in VS Code successfully!");
    Ok(())
}
