use chrono::{Local, NaiveDateTime};
use ratatui::text::{Line, Span};

pub struct CountdownState {
    pub target: Option<NaiveDateTime>,
    pub label: String,
    pub remaining_secs: Option<f64>,
    pub initial_secs: Option<f64>,
    pub reached: bool,
}

impl CountdownState {
    pub fn new() -> Self {
        Self {
            target: None,
            label: String::new(),
            remaining_secs: None,
            initial_secs: None,
            reached: false,
        }
    }

    pub fn set_target(&mut self, datetime: NaiveDateTime, label: String) {
        let now = Local::now().naive_local();
        let diff = datetime.signed_duration_since(now);
        let secs = diff.num_seconds() as f64;
        self.target = Some(datetime);
        self.label = label;
        self.remaining_secs = if secs > 0.0 { Some(secs) } else { Some(0.0) };
        self.initial_secs = if secs > 0.0 { Some(secs) } else { None };
        self.reached = secs <= 0.0;
    }

    pub fn update(&mut self) {
        if let Some(target) = self.target {
            let now = Local::now().naive_local();
            let diff = target.signed_duration_since(now);
            let secs = diff.num_seconds() as f64;
            self.reached = secs <= 0.0;
            self.remaining_secs = Some(if secs < 0.0 { 0.0 } else { secs });
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        if self.label.is_empty() {
            lines.push(Line::from(Span::raw("Countdown: (no target set)")));
            return lines;
        }

        let target = match self.target {
            Some(t) => t,
            None => {
                lines.push(Line::from(Span::raw("Countdown: (no target set)")));
                return lines;
            }
        };

        lines.push(Line::from(vec![
            Span::raw("Countdown to: "),
            Span::raw(self.label.clone()),
        ]));
        lines.push(Line::from(vec![
            Span::raw("Target: "),
            Span::raw(target.format("%Y-%m-%d %H:%M:%S").to_string()),
        ]));

        if self.reached {
            lines.push(Line::from(Span::raw("🎉 REACHED! 🎉")));
            return lines;
        }

        if let Some(secs) = self.remaining_secs {
            let total_secs = secs as u64;
            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let minutes = (total_secs % 3600) / 60;
            let seconds = total_secs % 60;

            let years = days / 365;
            let months = (days % 365) / 30;
            let rem_days = (days % 365) % 30;

            lines.push(Line::from(""));
            lines.push(Line::from(Span::raw("Time remaining:")));
            lines.push(Line::from(Span::raw(format!(
                "  {:3} years, {:2} months, {:2} days",
                years, months, rem_days
            ))));
            lines.push(Line::from(Span::raw(format!(
                "  {:2} days, {:02}:{:02}:{:02}",
                days, hours, minutes, seconds
            ))));

            if let Some(initial) = self.initial_secs {
                if initial > 0.0 {
                    let progress = 1.0 - (secs / initial);
                    let bar_width = 40;
                    let filled = (progress * bar_width as f64).round() as usize;
                    let filled = filled.min(bar_width);
                    let bar: String = std::iter::repeat('█')
                        .take(filled)
                        .chain(std::iter::repeat('░').take(bar_width - filled))
                        .collect();
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::raw(format!(
                        "  [{:.1}%] {}",
                        progress * 100.0,
                        bar
                    ))));
                }
            }
        }

        lines
    }
}
