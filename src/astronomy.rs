use chrono::Local;
use ratatui::text::{Line, Span};

pub struct AstronomyState {
    pub julian_day: Option<f64>,
    pub moon_phase: Option<f64>,
    pub moon_phase_name: Option<String>,
    pub season: Option<String>,
    pub days_since_new_moon: Option<f64>,
    pub days_until_new_moon: Option<f64>,
}

impl AstronomyState {
    pub fn new() -> Self {
        Self {
            julian_day: None,
            moon_phase: None,
            moon_phase_name: None,
            season: None,
            days_since_new_moon: None,
            days_until_new_moon: None,
        }
    }

    pub fn calculate(&mut self, holyc: &crate::ipc::holyc::HolyCManager) {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();

        match holyc.astronomy(&date_str) {
            Ok(ref map) => {
                self.julian_day = map
                    .get("julian_day")
                    .and_then(|v: &String| v.parse::<f64>().ok());
                self.moon_phase = map
                    .get("moon_phase")
                    .and_then(|v: &String| v.parse::<f64>().ok());
                self.moon_phase_name = map.get("moon_phase_name").cloned();
                self.season = map.get("season").cloned();
                self.days_since_new_moon = map
                    .get("days_since_new_moon")
                    .and_then(|v: &String| v.parse::<f64>().ok());
                self.days_until_new_moon = map
                    .get("days_until_new_moon")
                    .and_then(|v: &String| v.parse::<f64>().ok());
            }
            Err(_) => {
                self.julian_day = None;
                self.moon_phase = None;
                self.moon_phase_name = None;
                self.season = None;
                self.days_since_new_moon = None;
                self.days_until_new_moon = None;
            }
        }
    }

    fn moon_emoji(phase: f64) -> &'static str {
        let p = phase % 1.0;
        if p < 0.0625 || p >= 0.9375 {
            "🌑"
        } else if p < 0.1875 {
            "🌒"
        } else if p < 0.3125 {
            "🌓"
        } else if p < 0.4375 {
            "🌔"
        } else if p < 0.5625 {
            "🌕"
        } else if p < 0.6875 {
            "🌖"
        } else if p < 0.8125 {
            "🌗"
        } else {
            "🌘"
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        lines.push(Line::from(Span::raw("Astronomy:")));

        match self.julian_day {
            Some(jd) => lines.push(Line::from(Span::raw(format!(
                "  Julian Day:        {:.5}",
                jd
            )))),
            None => lines.push(Line::from(Span::raw(
                "  Julian Day:        (unavailable)",
            ))),
        }

        match self.moon_phase {
            Some(mp) => {
                let emoji = Self::moon_emoji(mp);
                let name = self
                    .moon_phase_name
                    .as_deref()
                    .unwrap_or("unknown");
                lines.push(Line::from(Span::raw(format!(
                    "  Moon phase:        {} {} ({:.2})",
                    emoji, name, mp
                ))));
            }
            None => lines.push(Line::from(Span::raw(
                "  Moon phase:        (unavailable)",
            ))),
        }

        match self.days_since_new_moon {
            Some(d) => lines.push(Line::from(Span::raw(format!(
                "  Days since new moon: {:.1}",
                d
            )))),
            None => lines.push(Line::from(Span::raw(
                "  Days since new moon: (unavailable)",
            ))),
        }

        match self.days_until_new_moon {
            Some(d) => lines.push(Line::from(Span::raw(format!(
                "  Days until new moon: {:.1}",
                d
            )))),
            None => lines.push(Line::from(Span::raw(
                "  Days until new moon: (unavailable)",
            ))),
        }

        match &self.season {
            Some(s) => lines.push(Line::from(Span::raw(format!("  Season:            {}", s)))),
            None => lines.push(Line::from(Span::raw(
                "  Season:            (unavailable)",
            ))),
        }

        lines
    }
}
