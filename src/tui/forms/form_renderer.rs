use ratatui::prelude::*;
use crate::config::{Theme, ColorTheme};
use crate::tui::components::{Input, Select, TextArea};

/// Shared rendering methods for form fields
pub struct FormRenderer;

impl FormRenderer {
    pub fn draw_minimal_input(f: &mut Frame, area: Rect, input: &Input, label: &str) {
        if area.height < 2 {
            return;
        }

        // Label on first line
        let label_style = if input.focused {
            Style::default()
                .fg(Theme::focused())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::text_muted())
        };

        let label_paragraph =
            ratatui::widgets::Paragraph::new(format!("{}:", label)).style(label_style);
        f.render_widget(label_paragraph, Rect::new(area.x, area.y, area.width, 1));

        // Input on second line
        let display_text = if input.value.is_empty() && !input.focused {
            input.placeholder.as_str()
        } else {
            &input.value
        };

        let input_style = if input.focused {
            Style::default().fg(Theme::text_primary()).bg(Theme::surface())
        } else if input.value.is_empty() {
            Style::default().fg(Theme::text_disabled())
        } else {
            Style::default().fg(Theme::text_primary())
        };

        let input_paragraph = ratatui::widgets::Paragraph::new(display_text).style(input_style);
        f.render_widget(
            input_paragraph,
            Rect::new(area.x, area.y + 1, area.width, 1),
        );

        // Draw subtle underline for focused field if there's space
        if input.focused && area.height >= 3 {
            let underline = "─".repeat(area.width as usize);
            let underline_paragraph = ratatui::widgets::Paragraph::new(underline)
                .style(Style::default().fg(Theme::focused()));
            f.render_widget(
                underline_paragraph,
                Rect::new(area.x, area.y + 2, area.width, 1),
            );
        }

        // Render cursor if focused
        if input.focused {
            let cursor_x = area.x + input.cursor_position as u16;
            let cursor_y = area.y + 1;
            if cursor_x < area.x + area.width && cursor_y < area.y + area.height {
                f.set_cursor(cursor_x, cursor_y);
            }
        }
    }

    pub fn draw_minimal_select<T: Clone>(
        f: &mut Frame,
        area: Rect,
        select: &Select<T>,
        label: &str,
    ) {
        if area.height < 2 {
            return;
        }

        // Label on first line
        let label_style = if select.is_focused() {
            Style::default()
                .fg(Theme::focused())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::text_muted())
        };

        let label_paragraph =
            ratatui::widgets::Paragraph::new(format!("{}:", label)).style(label_style);
        f.render_widget(label_paragraph, Rect::new(area.x, area.y, area.width, 1));

        // Show selected value on second line
        let selected_text = select.get_selected_label().unwrap_or("None".to_string());
        let selection_style = if select.is_focused() {
            Style::default().fg(Theme::text_primary()).bg(Theme::surface())
        } else {
            Style::default().fg(Theme::text_primary())
        };

        let selection_paragraph =
            ratatui::widgets::Paragraph::new(format!("▸ {}", selected_text)).style(selection_style);
        f.render_widget(
            selection_paragraph,
            Rect::new(area.x, area.y + 1, area.width, 1),
        );
    }

    pub fn draw_minimal_textarea(f: &mut Frame, area: Rect, textarea: &TextArea) {
        if area.height < 2 {
            return;
        }

        // Label on first line
        let label_style = if textarea.is_focused() {
            Style::default()
                .fg(Theme::focused())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::text_muted())
        };

        let label_paragraph = ratatui::widgets::Paragraph::new("Notes:").style(label_style);
        f.render_widget(label_paragraph, Rect::new(area.x, area.y, area.width, 1));

        // Content area
        let content_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);

        let content = if textarea.content().is_empty() && !textarea.is_focused() {
            "Add notes..."
        } else {
            textarea.content()
        };

        let content_style = if textarea.is_focused() {
            Style::default().fg(Theme::text_primary()).bg(Theme::surface())
        } else if textarea.content().is_empty() {
            Style::default().fg(Theme::text_disabled())
        } else {
            Style::default().fg(Theme::text_primary())
        };

        let content_paragraph = ratatui::widgets::Paragraph::new(content)
            .style(content_style)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(content_paragraph, content_area);
    }

    pub fn draw_minimal_search_input(f: &mut Frame, area: Rect, input: &Input) {
        if area.height < 2 {
            return;
        }

        // Label on first line
        let label_style = if input.focused {
            Style::default()
                .fg(Theme::focused())
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Theme::text_muted())
        };

        let label_paragraph = ratatui::widgets::Paragraph::new("Search:").style(label_style);
        f.render_widget(label_paragraph, Rect::new(area.x, area.y, area.width, 1));

        // Input on second line
        let display_text = if input.value.is_empty() && !input.focused {
            input.placeholder.as_str()
        } else {
            &input.value
        };

        let input_style = if input.focused {
            Style::default().fg(Theme::text_primary()).bg(Theme::surface())
        } else if input.value.is_empty() {
            Style::default().fg(Theme::text_disabled())
        } else {
            Style::default().fg(Theme::text_primary())
        };

        let input_paragraph = ratatui::widgets::Paragraph::new(display_text).style(input_style);
        f.render_widget(
            input_paragraph,
            Rect::new(area.x, area.y + 1, area.width, 1),
        );

        // Render cursor if focused
        if input.focused {
            let cursor_x = area.x + input.cursor_position as u16;
            let cursor_y = area.y + 1;
            if cursor_x < area.x + area.width && cursor_y < area.y + area.height {
                f.set_cursor(cursor_x, cursor_y);
            }
        }
    }
}
