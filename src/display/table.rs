use crate::config::colors::ApolloRgb;
use crate::core::Todo;
use colored::*;

pub fn format_list(todos: &[&Todo]) {
    if todos.is_empty() {
        println!("No TODOs found.");
        return;
    }

    // Print header with Apollo color styling
    println!(
        "{}",
        format!(
            "{:<4} │ {:<3} │ {:<1} │ {:<10} │ {:<40} │ {:<15}",
            "ID", "PRI", "S", "Category", "Title", "Tags"
        )
        .truecolor(
            ApolloRgb::LIGHT_CREAM.0,
            ApolloRgb::LIGHT_CREAM.1,
            ApolloRgb::LIGHT_CREAM.2
        )
        .bold()
    );

    let separator = "─".repeat(80);
    println!(
        "{}",
        separator.truecolor(
            ApolloRgb::MED_GRAY.0,
            ApolloRgb::MED_GRAY.1,
            ApolloRgb::MED_GRAY.2
        )
    );

    // Print todos
    for todo in todos {
        let category = todo.category.as_deref().unwrap_or("-");
        let tags = if todo.tags.is_empty() {
            "-".to_string()
        } else {
            todo.tags.join(",")
        };

        let title = if todo.title.len() > 40 {
            format!("{}...", &todo.title[..37])
        } else {
            todo.title.clone()
        };

        // Format ID with dimmed style
        let id_str = format!("{:<4}", todo.id).truecolor(
            ApolloRgb::MED_GRAY.0,
            ApolloRgb::MED_GRAY.1,
            ApolloRgb::MED_GRAY.2,
        );

        // Format priority with color
        let priority_str = format!("{:<3}", format_priority(&todo.priority)).truecolor(
            get_priority_color(&todo.priority).0,
            get_priority_color(&todo.priority).1,
            get_priority_color(&todo.priority).2,
        );

        // Format status icon with color
        let status_icon = get_status_icon(&todo.status).truecolor(
            get_status_color(&todo.status).0,
            get_status_color(&todo.status).1,
            get_status_color(&todo.status).2,
        );

        // Format other fields
        let category_str = format!("{:<10}", category).truecolor(
            ApolloRgb::CREAM.0,
            ApolloRgb::CREAM.1,
            ApolloRgb::CREAM.2,
        );

        let title_str = format!("{:<40}", title).truecolor(
            ApolloRgb::LIGHT_CREAM.0,
            ApolloRgb::LIGHT_CREAM.1,
            ApolloRgb::LIGHT_CREAM.2,
        );

        let tags_str = format!("{:<15}", tags).truecolor(
            ApolloRgb::CREAM.0,
            ApolloRgb::CREAM.1,
            ApolloRgb::CREAM.2,
        );

        // Separators
        let sep = "│".truecolor(
            ApolloRgb::MED_GRAY.0,
            ApolloRgb::MED_GRAY.1,
            ApolloRgb::MED_GRAY.2,
        );

        println!(
            "{} {} {} {} {} {} {} {} {} {} {}",
            id_str,
            sep,
            priority_str,
            sep,
            status_icon,
            sep,
            category_str,
            sep,
            title_str,
            sep,
            tags_str
        );
    }

    println!();
    println!(
        "{}",
        format!("Showing {} TODOs", todos.len()).truecolor(
            ApolloRgb::CREAM.0,
            ApolloRgb::CREAM.1,
            ApolloRgb::CREAM.2
        )
    );
}

fn format_priority(priority: &crate::core::Priority) -> String {
    match priority {
        crate::core::Priority::P0 => "P0".to_string(),
        crate::core::Priority::P1 => "P1".to_string(),
        crate::core::Priority::P2 => "P2".to_string(),
        crate::core::Priority::P3 => "P3".to_string(),
        crate::core::Priority::P4 => "P4".to_string(),
        crate::core::Priority::P5 => "P5".to_string(),
    }
}

fn get_priority_color(priority: &crate::core::Priority) -> (u8, u8, u8) {
    match priority {
        crate::core::Priority::P0 => ApolloRgb::BRIGHT_MAGENTA, // Urgent
        crate::core::Priority::P1 => ApolloRgb::BRIGHT_ORANGE,  // High
        crate::core::Priority::P2 => ApolloRgb::YELLOW,         // Medium
        crate::core::Priority::P3 | crate::core::Priority::P4 => ApolloRgb::PALE_GREEN, // Low
        crate::core::Priority::P5 => ApolloRgb::MED_GRAY,       // Wishlist
    }
}

fn get_status_icon(status: &crate::core::Status) -> String {
    match status {
        crate::core::Status::Todo => "T".to_string(),
        crate::core::Status::InProgress => "W".to_string(),
        crate::core::Status::Done => "D".to_string(),
        crate::core::Status::Archived => "A".to_string(),
    }
}

fn get_status_color(status: &crate::core::Status) -> (u8, u8, u8) {
    match status {
        crate::core::Status::Todo => ApolloRgb::SKY_BLUE, // Todo
        crate::core::Status::InProgress => ApolloRgb::YELLOW, // In Progress
        crate::core::Status::Done => ApolloRgb::BRIGHT_GREEN, // Done
        crate::core::Status::Archived => ApolloRgb::MED_GRAY, // Archived
    }
}
