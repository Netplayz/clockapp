use std::time::{Duration, Instant};

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub struct TimerInstance {
    pub label: String,
    pub total_duration: Duration,
    pub remaining: Duration,
    pub running: bool,
    pub paused: bool,
    pub start_time: Option<Instant>,
}

pub struct TimerState {
    pub timers: Vec<TimerInstance>,
    pub selected: usize,
}

impl TimerState {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
            selected: 0,
        }
    }

    pub fn add_timer(&mut self, label: String, seconds: u64) {
        let duration = Duration::from_secs(seconds);
        self.timers.push(TimerInstance {
            label,
            total_duration: duration,
            remaining: duration,
            running: false,
            paused: false,
            start_time: None,
        });
    }

    pub fn remove_timer(&mut self, idx: usize) {
        if idx < self.timers.len() {
            self.timers.remove(idx);
            if self.selected >= self.timers.len() && !self.timers.is_empty() {
                self.selected = self.timers.len() - 1;
            }
        }
    }

    pub fn start_timer(&mut self, idx: usize) {
        if let Some(t) = self.timers.get_mut(idx) {
            if !t.running {
                t.running = true;
                t.paused = false;
                t.start_time = Some(Instant::now());
            }
        }
    }

    pub fn pause_timer(&mut self, idx: usize) {
        if let Some(t) = self.timers.get_mut(idx) {
            if t.running && !t.paused {
                if let Some(start) = t.start_time {
                    t.remaining = t.remaining.saturating_sub(start.elapsed());
                }
                t.paused = true;
                t.start_time = None;
            }
        }
    }

    pub fn reset_timer(&mut self, idx: usize) {
        if let Some(t) = self.timers.get_mut(idx) {
            t.remaining = t.total_duration;
            t.running = false;
            t.paused = false;
            t.start_time = None;
        }
    }

    pub fn update(&mut self) {
        for t in &mut self.timers {
            if t.running && !t.paused {
                if let Some(start) = t.start_time {
                    t.remaining = t.total_duration.saturating_sub(start.elapsed());
                    if t.remaining.is_zero() {
                        t.running = false;
                        t.start_time = None;
                    }
                }
            }
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        if self.timers.is_empty() {
            lines.push(Line::from(Span::raw("No timers. Press 'a' to add.")));
            return lines;
        }

        for (i, timer) in self.timers.iter().enumerate() {
            let prefix = if i == self.selected { "▸ " } else { "  " };

            let total_secs = timer.total_duration.as_secs_f64();
            let remaining_secs = timer.remaining.as_secs_f64();
            let progress = if total_secs > 0.0 {
                1.0 - (remaining_secs / total_secs)
            } else {
                0.0
            };

            let bar_width = 20;
            let filled = (progress * bar_width as f64) as usize;
            let filled = filled.min(bar_width);
            let bar: String =
                format!("[{}{}]", "█".repeat(filled), "░".repeat(bar_width.saturating_sub(filled)));

            let status = if timer.running {
                if timer.paused { "⏸" } else { "▶" }
            } else {
                "⏹"
            };

            let label = if timer.label.is_empty() {
                format!("Timer {}", i + 1)
            } else {
                timer.label.clone()
            };

            let remaining_str = format_remaining(timer.remaining);

            let line_style = if i == self.selected {
                Style::new()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::new().fg(Color::White)
            };

            lines.push(Line::from(vec![
                Span::styled(prefix.to_string(), line_style),
                Span::styled(format!("{} ", status), line_style),
                Span::styled(label, Style::new().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(" ".to_string(), Style::new()),
                Span::styled(bar, Style::new().fg(Color::White)),
                Span::styled(
                    format!(" {:.0}%", progress * 100.0),
                    Style::new().fg(Color::White),
                ),
                Span::styled(
                    format!(" ({})", remaining_str),
                    Style::new().fg(Color::Yellow),
                ),
            ]));
        }

        lines
    }
}

fn format_remaining(d: Duration) -> String {
    let total_secs = d.as_secs();
    let secs = total_secs % 60;
    let mins = (total_secs / 60) % 60;
    let hours = total_secs / 3600;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}
