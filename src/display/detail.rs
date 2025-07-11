use crate::core::Todo;
use colored::*;

pub fn format_detail(todo: &Todo) {
    println!("{}", format!("TODO #{}", todo.id).bold().blue());
    println!("{}: {}", "Title".bold(), todo.title);
    println!("{}: {}", "Status".bold(), format_status(&todo.status));
    println!("{}: {}", "Priority".bold(), format_priority(&todo.priority));

    if let Some(ref category) = todo.category {
        println!("{}: {}", "Category".bold(), category);
    }

    if let Some(ref project) = todo.project {
        println!("{}: {}", "Project".bold(), project);
    }

    if !todo.tags.is_empty() {
        println!("{}: {}", "Tags".bold(), todo.tags.join(", "));
    }

    println!(
        "{}: {}",
        "Created".bold(),
        todo.created_date.format("%Y-%m-%d %H:%M:%S")
    );

    if let Some(finished_date) = todo.finished_date {
        println!(
            "{}: {}",
            "Finished".bold(),
            finished_date.format("%Y-%m-%d %H:%M:%S")
        );
    } else {
        println!("{}: -", "Finished".bold());
    }

    if let Some(ref notes) = todo.notes {
        println!("{}: ", "Notes".bold());
        for line in notes.lines() {
            println!("  {}", line);
        }
    }
}

fn format_priority(priority: &crate::core::Priority) -> colored::ColoredString {
    match priority {
        crate::core::Priority::P0 => "P0 (Urgent)".red().bold(),
        crate::core::Priority::P1 => "P1 (Must have)".yellow().bold(),
        crate::core::Priority::P2 => "P2 (Should do)".normal(),
        crate::core::Priority::P3 => "P3 (Nice to have)".blue(),
        crate::core::Priority::P4 => "P4 (Wishlist)".cyan(),
        crate::core::Priority::P5 => "P5 (Worth considering)".dimmed(),
    }
}

fn format_status(status: &crate::core::Status) -> colored::ColoredString {
    match status {
        crate::core::Status::Todo => "Todo".normal(),
        crate::core::Status::InProgress => "In Progress".yellow(),
        crate::core::Status::Done => "Done".green(),
        crate::core::Status::Archived => "Archived".dimmed(),
    }
}
