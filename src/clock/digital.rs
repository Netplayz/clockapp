use chrono::{Local, Datelike};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub struct DigitalClockState {
    pub twenty_four_hour: bool,
    pub show_seconds: bool,
}

impl DigitalClockState {
    pub fn new() -> Self {
        Self {
            twenty_four_hour: true,
            show_seconds: true,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&self) -> Vec<Line<'static>> {
        let now = Local::now();
        let mut lines = Vec::new();

        let time_str = if self.twenty_four_hour {
            if self.show_seconds {
                now.format("%H:%M:%S").to_string()
            } else {
                now.format("%H:%M").to_string()
            }
        } else if self.show_seconds {
            now.format("%I:%M:%S %p").to_string()
        } else {
            now.format("%I:%M %p").to_string()
        };

        lines.push(Line::from(Span::styled(
            time_str,
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));

        lines.push(Line::from(Span::styled(
            now.format("%A, %B %d, %Y").to_string(),
            Style::new().fg(Color::Green),
        )));

        lines.push(Line::from(Span::styled(
            format!("Timezone: {}", now.format("%Z")),
            Style::new().fg(Color::Yellow),
        )));

        lines.push(Line::from(Span::styled(
            format!("Unix Epoch: {}", now.timestamp()),
            Style::new().fg(Color::White),
        )));

        lines.push(Line::from(Span::styled(
            format!("ISO 8601: {}", now.format("%Y-%m-%dT%H:%M:%S%z")),
            Style::new().fg(Color::White),
        )));

        lines.push(Line::from(Span::styled(
            format!("RFC 2822: {}", now.format("%a, %d %b %Y %H:%M:%S %z")),
            Style::new().fg(Color::White),
        )));

        lines.push(Line::from(Span::styled(
            format!("Day of Year: {}", now.ordinal()),
            Style::new().fg(Color::White),
        )));

        lines.push(Line::from(Span::styled(
            format!("Week Number: {}", now.iso_week().week()),
            Style::new().fg(Color::White),
        )));

        lines
    }
}
