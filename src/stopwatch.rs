use std::time::{Duration, Instant};

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub struct Lap {
    pub number: u32,
    pub split_time: Duration,
    pub lap_time: Duration,
}

pub struct StopwatchState {
    pub running: bool,
    pub start_time: Option<Instant>,
    pub elapsed: Duration,
    pub laps: Vec<Lap>,
    pub last_lap_time: Option<Instant>,
}

impl StopwatchState {
    pub fn new() -> Self {
        Self {
            running: false,
            start_time: None,
            elapsed: Duration::ZERO,
            laps: Vec::new(),
            last_lap_time: None,
        }
    }

    pub fn start(&mut self) {
        if !self.running {
            self.running = true;
            self.start_time = Some(Instant::now());
            self.last_lap_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if self.running {
            self.running = false;
            if let Some(start) = self.start_time {
                self.elapsed += start.elapsed();
            }
            self.start_time = None;
            self.last_lap_time = None;
        }
    }

    pub fn lap(&mut self) {
        if self.running {
            let now = Instant::now();
            let split = self.elapsed
                + self.start_time.map(|s| s.elapsed()).unwrap_or_default();
            let lap_dur = self
                .last_lap_time
                .map(|t| t.elapsed())
                .unwrap_or_default();
            let lap_num = self.laps.len() as u32 + 1;
            self.laps.push(Lap {
                number: lap_num,
                split_time: split,
                lap_time: lap_dur,
            });
            self.last_lap_time = Some(now);
        }
    }

    pub fn reset(&mut self) {
        self.running = false;
        self.start_time = None;
        self.elapsed = Duration::ZERO;
        self.laps.clear();
        self.last_lap_time = None;
    }

    pub fn update(&mut self) {}

    fn current_elapsed(&self) -> Duration {
        if self.running {
            self.elapsed
                + self
                    .start_time
                    .map(|s| s.elapsed())
                    .unwrap_or_default()
        } else {
            self.elapsed
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        let elapsed = self.current_elapsed();

        let total_ms = elapsed.as_millis();
        let ms = total_ms % 1000;
        let total_secs = total_ms / 1000;
        let secs = total_secs % 60;
        let mins = (total_secs / 60) % 60;
        let hours = total_secs / 3600;
        let time_str = format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, ms);

        lines.push(Line::from(Span::styled(
            time_str,
            Style::new()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));

        lines.push(Line::from(Span::styled(
            "[S]tart  [L]ap  [R]eset".to_string(),
            Style::new().fg(Color::Yellow),
        )));

        lines.push(Line::from(Span::raw("")));

        if !self.laps.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Lap", Style::new().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("  ", Style::new()),
                Span::styled(
                    "Split        ",
                    Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Lap Time",
                    Style::new().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ]));

            for lap in self.laps.iter().rev().take(10) {
                let split_str = format_duration(&lap.split_time);
                let lap_str = format_duration(&lap.lap_time);
                lines.push(Line::from(vec![
                    Span::styled(format!("{:>3}  ", lap.number), Style::new().fg(Color::White)),
                    Span::styled(split_str, Style::new().fg(Color::White)),
                    Span::styled("  ", Style::new()),
                    Span::styled(lap_str, Style::new().fg(Color::Cyan)),
                ]));
            }
        }

        lines
    }
}

fn format_duration(d: &Duration) -> String {
    let total_ms = d.as_millis();
    let ms = total_ms % 1000;
    let total_secs = total_ms / 1000;
    let secs = total_secs % 60;
    let mins = (total_secs / 60) % 60;
    let hours = total_secs / 3600;
    format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, ms)
}
