use chrono::{Duration, NaiveDate};
use ratatui::text::{Line, Span};

pub enum TimeMathOp {
    Diff,
    Add,
    WeekOf,
    Age,
}

pub struct TimeMathState {
    pub operation: TimeMathOp,
    pub date1: String,
    pub date2: String,
    pub duration_days: i64,
    pub result: Option<String>,
}

impl TimeMathState {
    pub fn new() -> Self {
        Self {
            operation: TimeMathOp::Diff,
            date1: String::new(),
            date2: String::new(),
            duration_days: 0,
            result: None,
        }
    }

    pub fn compute_diff(&mut self, date1: &str, date2: &str) {
        let d1 = match NaiveDate::parse_from_str(date1, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                self.result = Some(format!("Error parsing date1: {}", e));
                return;
            }
        };
        let d2 = match NaiveDate::parse_from_str(date2, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                self.result = Some(format!("Error parsing date2: {}", e));
                return;
            }
        };
        let diff = (d2 - d1).num_days();
        let abs_diff = diff.abs();
        let years = abs_diff / 365;
        let months = (abs_diff % 365) / 30;
        let days = (abs_diff % 365) % 30;

        self.result = Some(format!(
            "{} days ({} years, {} months, {} days){}",
            abs_diff,
            years,
            months,
            days,
            if diff < 0 { " (date1 is later)" } else { "" }
        ));
    }

    pub fn compute_add(&mut self, date: &str, days: i64) {
        let d = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                self.result = Some(format!("Error parsing date: {}", e));
                return;
            }
        };
        let result = d + Duration::days(days);
        self.result = Some(format!(
            "{} + {} days = {}",
            date,
            days,
            result.format("%Y-%m-%d")
        ));
    }

    pub fn compute_weekof(&mut self, date: &str) {
        let d = match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                self.result = Some(format!("Error parsing date: {}", e));
                return;
            }
        };
        let iso_week = d.format("%V").to_string();
        let year = d.format("%G").to_string();
        let weekday = d.format("%A").to_string();
        self.result = Some(format!(
            "{} is {} (ISO week {}) in year {}",
            date, weekday, iso_week, year
        ));
    }

    pub fn compute_age(&mut self, birthday: &str) {
        let bd = match NaiveDate::parse_from_str(birthday, "%Y-%m-%d") {
            Ok(d) => d,
            Err(e) => {
                self.result = Some(format!("Error parsing birthday: {}", e));
                return;
            }
        };
        let today = chrono::Local::now().naive_local().date();
        let diff = today - bd;
        let days = diff.num_days();
        if days < 0 {
            self.result = Some("Birthday is in the future!".to_string());
            return;
        }
        let years = days / 365;
        let rem_days = days % 365;
        let months = rem_days / 30;
        let rem = rem_days % 30;
        self.result = Some(format!(
            "Age: {} years, {} months, {} days ({} total days)",
            years, months, rem, days
        ));
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        let op_name = match self.operation {
            TimeMathOp::Diff => "Date Difference",
            TimeMathOp::Add => "Add Days",
            TimeMathOp::WeekOf => "Week Of",
            TimeMathOp::Age => "Age Calculator",
        };

        lines.push(Line::from(Span::raw(format!("Time Math: {}", op_name))));
        lines.push(Line::from(""));

        match self.operation {
            TimeMathOp::Diff => {
                lines.push(Line::from(Span::raw(format!("  Date 1: {}", self.date1))));
                lines.push(Line::from(Span::raw(format!("  Date 2: {}", self.date2))));
                lines.push(Line::from(Span::raw(
                    "  Compute the difference between two dates.",
                )));
            }
            TimeMathOp::Add => {
                lines.push(Line::from(Span::raw(format!("  Date:  {}", self.date1))));
                lines.push(Line::from(Span::raw(format!(
                    "  Days:  {}",
                    self.duration_days
                ))));
                lines.push(Line::from(Span::raw(
                    "  Add or subtract days from a date.",
                )));
            }
            TimeMathOp::WeekOf => {
                lines.push(Line::from(Span::raw(format!("  Date:  {}", self.date1))));
                lines.push(Line::from(Span::raw(
                    "  Find the ISO week number for a date.",
                )));
            }
            TimeMathOp::Age => {
                lines.push(Line::from(Span::raw(format!(
                    "  Birthday: {}",
                    self.date1
                ))));
                lines.push(Line::from(Span::raw(
                    "  Calculate age from a birth date.",
                )));
            }
        }

        lines.push(Line::from(""));
        match &self.result {
            Some(r) => {
                lines.push(Line::from(Span::raw("Result:")));
                lines.push(Line::from(Span::raw(format!("  {}", r))));
            }
            None => {
                lines.push(Line::from(Span::raw("Result: (not computed)")));
            }
        }

        lines
    }
}
