use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

/// Git status information for the data directory
#[derive(Debug, Clone)]
pub struct GitStatus {
    pub has_changes: bool,
    pub pretty_path: String,
    pub status_message: String,
}

/// Get git status for the guidebook data directory
pub fn get_git_status() -> Result<GitStatus> {
    let data_dir = get_data_dir()?;
    
    // Check if this is a git repository
    let git_dir = data_dir.join(".git");
    if !git_dir.exists() {
        return Ok(GitStatus {
            has_changes: false,
            pretty_path: format_pretty_path(&data_dir)?,
            status_message: "not a git repository".to_string(),
        });
    }
    
    // Run git status --porcelain to check for changes
    let output = Command::new("git")
        .args(&["status", "--porcelain"])
        .current_dir(&data_dir)
        .output()?;
    
    if !output.status.success() {
        return Ok(GitStatus {
            has_changes: false,
            pretty_path: format_pretty_path(&data_dir)?,
            status_message: "git error".to_string(),
        });
    }
    
    let status_output = String::from_utf8_lossy(&output.stdout);
    let has_changes = !status_output.trim().is_empty();
    
    let status_message = if has_changes {
        "modified".to_string()
    } else {
        "up to date".to_string()
    };
    
    Ok(GitStatus {
        has_changes,
        pretty_path: format_pretty_path(&data_dir)?,
        status_message,
    })
}

/// Get the guidebook data directory
fn get_data_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    
    Ok(home.join(".local").join("share").join("guidebook"))
}

/// Format a path with ~ for home directory
fn format_pretty_path(path: &PathBuf) -> Result<String> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    
    let path_str = path.to_string_lossy();
    let home_str = home.to_string_lossy();
    
    if path_str.starts_with(&*home_str) {
        Ok(path_str.replace(&*home_str, "~"))
    } else {
        Ok(path_str.to_string())
    }
}
