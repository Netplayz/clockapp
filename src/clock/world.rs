use chrono::Utc;
use chrono_tz::Tz;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub struct WorldCityDisplay {
    pub name: String,
    pub timezone: Tz,
}

pub struct WorldCityState {
    pub cities: Vec<WorldCityDisplay>,
}

impl WorldCityState {
    pub fn new(cities: Vec<WorldCityDisplay>) -> Self {
        Self { cities }
    }

    pub fn update(&mut self) {}

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let now_utc = Utc::now();

        for city in &self.cities {
            let local = now_utc.with_timezone(&city.timezone);
            let time_str = local.format("%H:%M:%S").to_string();
            let offset_str = format!("UTC{}", local.format("%:z"));

            lines.push(Line::from(vec![
                Span::styled(
                    format!("{:20}", city.name),
                    Style::new()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ", Style::new()),
                Span::styled(time_str, Style::new().fg(Color::Green)),
                Span::styled("  ", Style::new()),
                Span::styled(offset_str, Style::new().fg(Color::Yellow)),
            ]));
        }

        lines
    }
}
