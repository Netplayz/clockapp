use crate::storage::AlarmConfig;
use chrono::{Local, Timelike};
use ratatui::text::{Line, Span};

pub struct AlarmState {
    pub alarms: Vec<AlarmConfig>,
    pub triggered: Vec<String>,
}

impl AlarmState {
    pub fn new() -> Self {
        Self {
            alarms: Vec::new(),
            triggered: Vec::new(),
        }
    }

    pub fn check_alarms(&mut self) {
        let now = Local::now();
        let current_hour = now.hour() as u8;
        let current_minute = now.minute() as u8;
        let current_second = now.second() as u8;
        let current_dow = now.format("%u").to_string().parse::<u8>().unwrap_or(0);

        for alarm in &self.alarms {
            if !alarm.enabled {
                continue;
            }
            if alarm.hour == current_hour
                && alarm.minute == current_minute
                && alarm.second == current_second
            {
                if alarm.recurring {
                    if alarm.days_of_week.is_empty() || alarm.days_of_week.contains(&current_dow) {
                        if !self.triggered.contains(&alarm.label) {
                            self.triggered.push(alarm.label.clone());
                        }
                    }
                } else {
                    if !self.triggered.contains(&alarm.label) {
                        self.triggered.push(alarm.label.clone());
                    }
                }
            }
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        lines.push(Line::from(Span::raw("Alarms:")));

        if self.alarms.is_empty() {
            lines.push(Line::from(Span::raw("  (no alarms configured)")));
            return lines;
        }

        for (i, alarm) in self.alarms.iter().enumerate() {
            let hour_str = format!("{:02}", alarm.hour);
            let min_str = format!("{:02}", alarm.minute);
            let sec_str = format!("{:02}", alarm.second);
            let time_str = format!("{}:{}:{}", hour_str, min_str, sec_str);

            let recurring = if alarm.recurring {
                    let days: Vec<String> = alarm
                    .days_of_week
                    .iter()
                    .map(|d| {
                        match d {
                            1 => "Mon".to_string(),
                            2 => "Tue".to_string(),
                            3 => "Wed".to_string(),
                            4 => "Thu".to_string(),
                            5 => "Fri".to_string(),
                            6 => "Sat".to_string(),
                            7 => "Sun".to_string(),
                            _ => "?".to_string(),
                        }
                    })
                    .collect();
                if days.is_empty() {
                    " daily".to_string()
                } else {
                    format!(" {}", days.join(","))
                }
            } else {
                " once".to_string()
            };

            let status = if alarm.enabled { "ON" } else { "OFF" };
            lines.push(Line::from(vec![
                Span::raw(format!("  {}. ", i + 1)),
                Span::raw(time_str),
                Span::raw(recurring),
                Span::raw(format!(" [{}]", status)),
                Span::raw(format!(" {}", alarm.label)),
            ]));
        }

        if !self.triggered.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::raw("Triggered alarms:")));
            for label in &self.triggered {
                lines.push(Line::from(vec![
                    Span::raw("  🔔 "),
                    Span::raw(label.clone()),
                ]));
            }
        }

        lines
    }
}
