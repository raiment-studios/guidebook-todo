use crate::core::{Priority, Status, Todo};

pub fn search_todos<'a>(todos: &'a [Todo], query: &str) -> Vec<&'a Todo> {
    let query_lower = query.to_lowercase();

    todos
        .iter()
        .filter(|todo| {
            // Exclude archived TODOs by default
            if todo.status == Status::Archived {
                return false;
            }

            // Search in title
            if todo.title.to_lowercase().contains(&query_lower) {
                return true;
            }

            // Search in notes
            if let Some(ref notes) = todo.notes {
                if notes.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }

            // Search in tags
            if todo
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(&query_lower))
            {
                return true;
            }

            // Search in category
            if let Some(ref category) = todo.category {
                if category.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }

            // Search in project
            if let Some(ref project) = todo.project {
                if project.to_lowercase().contains(&query_lower) {
                    return true;
                }
            }

            false
        })
        .collect()
}

pub fn filter_by_special_syntax<'a>(todos: &'a [Todo], query: &str) -> Vec<&'a Todo> {
    if query.starts_with('#') {
        // Tag filter: #bug
        let tag = query[1..].to_lowercase();
        return todos
            .iter()
            .filter(|todo| todo.tags.iter().any(|t| t == &tag))
            .collect();
    }

    if query.starts_with('@') {
        // Category filter: @work
        let category = query[1..].to_lowercase();
        return todos
            .iter()
            .filter(|todo| {
                todo.category
                    .as_ref()
                    .map_or(false, |cat| cat.to_lowercase() == category)
            })
            .collect();
    }

    if query.starts_with('!') {
        // Status filter: !done
        let status_str = query[1..].to_lowercase();
        if let Ok(status) = parse_status_for_filter(&status_str) {
            return todos.iter().filter(|todo| todo.status == status).collect();
        }
    }

    if query.starts_with('p') && query.len() == 2 {
        // Priority filter: p0, p1, etc.
        if let Ok(priority) = parse_priority_for_filter(query) {
            return todos
                .iter()
                .filter(|todo| todo.priority == priority)
                .collect();
        }
    }

    // Fallback to regular search
    search_todos(todos, query)
}

fn parse_status_for_filter(status_str: &str) -> Result<Status, ()> {
    match status_str {
        "todo" => Ok(Status::Todo),
        "inprogress" | "in-progress" | "in_progress" => Ok(Status::InProgress),
        "done" => Ok(Status::Done),
        "archived" => Ok(Status::Archived),
        _ => Err(()),
    }
}

fn parse_priority_for_filter(priority_str: &str) -> Result<Priority, ()> {
    match priority_str {
        "p0" => Ok(Priority::P0),
        "p1" => Ok(Priority::P1),
        "p2" => Ok(Priority::P2),
        "p3" => Ok(Priority::P3),
        "p4" => Ok(Priority::P4),
        "p5" => Ok(Priority::P5),
        _ => Err(()),
    }
}

pub fn sort_todos_by_priority(todos: &mut [&Todo]) {
    todos.sort_by(|a, b| {
        // Sort by priority first (P0 is highest)
        match a.priority_value().cmp(&b.priority_value()) {
            std::cmp::Ordering::Equal => {
                // Then by creation date (newest first)
                b.created_date.cmp(&a.created_date)
            }
            other => other,
        }
    });
}

pub fn sort_todos_by_date(todos: &mut [&Todo]) {
    todos.sort_by(|a, b| b.created_date.cmp(&a.created_date));
}

pub fn get_active_todos<'a>(todos: &'a [Todo]) -> Vec<&'a Todo> {
    todos
        .iter()
        .filter(|todo| todo.is_active() && todo.status != Status::Archived)
        .collect()
}

pub fn get_all_todos_including_archived<'a>(todos: &'a [Todo]) -> Vec<&'a Todo> {
    todos.iter().collect()
}
