use chrono::{Datelike, Local, NaiveDate};
use ratatui::text::{Line, Span};

pub struct CalendarState {
    pub current_date: NaiveDate,
    pub view_year: i32,
    pub view_month: u32,
    pub events: Vec<(NaiveDate, String)>,
}

impl CalendarState {
    pub fn new() -> Self {
        let today = Local::now().naive_local().date();
        Self {
            current_date: today,
            view_year: today.year(),
            view_month: today.month(),
            events: Vec::new(),
        }
    }

    pub fn next_month(&mut self) {
        if self.view_month == 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
    }

    pub fn prev_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
    }

    pub fn update(&mut self) {
        self.current_date = Local::now().naive_local().date();
    }

    fn days_in_month(year: i32, month: u32) -> u32 {
        let (y, m) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
        let first_of_next =
            NaiveDate::from_ymd_opt(y, m, 1).unwrap_or_else(|| NaiveDate::from_ymd_opt(year, 12, 31).unwrap());
        let first_of_this =
            NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        (first_of_next - first_of_this).num_days() as u32
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        let month_name = match self.view_month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "?",
        };

        let header = format!("{} {}", month_name, self.view_year);
        let pad = (36usize.saturating_sub(header.len())) / 2;
        lines.push(Line::from(Span::raw(format!(
            "{}{}",
            " ".repeat(pad),
            header
        ))));
        lines.push(Line::from(Span::raw("  Wk Su Mo Tu We Th Fr Sa")));

        let first = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1).unwrap();
        let start_dow = first.format("%w").to_string().parse::<u32>().unwrap_or(0);
        let days_in = Self::days_in_month(self.view_year, self.view_month);

        let mut day = 1i32;
        for week in 0..6 {
            if day > days_in as i32 {
                break;
            }
            let week_num = first
                .checked_add_signed(chrono::Duration::days((week * 7) as i64))
                .map(|d| d.format("%V").to_string())
                .unwrap_or_default();

            let mut week_line = format!("{:>3} ", week_num);

            for dow in 0..7 {
                if (week == 0 && dow < start_dow as i32) || day > days_in as i32 {
                    week_line.push_str("   ");
                } else {
                    let date = NaiveDate::from_ymd_opt(self.view_year, self.view_month, day as u32).unwrap();
                    let event_count = self
                        .events
                        .iter()
                        .filter(|(ed, _)| *ed == date)
                        .count();
                    let day_str = if date == self.current_date {
                        format!("[{:02}]", day)
                    } else if event_count > 0 {
                        format!("{:02}*", day)
                    } else {
                        format!(" {:02} ", day)
                    };
                    week_line.push_str(&day_str);
                    day += 1;
                }
            }
            lines.push(Line::from(Span::raw(week_line)));
        }

        lines
    }
}
