use crate::core::Priority;

// Fixed application defaults (no user configuration files)

pub const DEFAULT_PRIORITY: Priority = Priority::P2;
pub const MAX_TITLE_LENGTH: usize = 200;
pub const MAX_CATEGORY_LENGTH: usize = 50;
pub const MAX_PROJECT_LENGTH: usize = 100;
pub const MAX_NOTES_LENGTH: usize = 2000;

// Display defaults
pub const DATE_FORMAT: &str = "%Y-%m-%d %H:%M";
pub const SHOW_IDS: bool = true;
pub const USE_COLORS: bool = true;

// Filter defaults
pub const HIDE_DONE_BY_DEFAULT: bool = true;
