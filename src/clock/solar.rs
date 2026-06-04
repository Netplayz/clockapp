use chrono::Local;
use ratatui::text::{Line, Span};

pub struct SolarState {
    pub lat: f64,
    pub lon: f64,
    pub sunrise: Option<String>,
    pub sunset: Option<String>,
    pub solar_noon: Option<String>,
    pub day_length: Option<f64>,
    pub equation_of_time: Option<f64>,
}

impl SolarState {
    pub fn new() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            sunrise: None,
            sunset: None,
            solar_noon: None,
            day_length: None,
            equation_of_time: None,
        }
    }

    pub fn calculate(&mut self, holyc: &crate::ipc::holyc::HolyCManager) {
        let now = Local::now();
        let date_str = now.format("%Y-%m-%d").to_string();
        let time_str = now.format("%H:%M:%S").to_string();
        let datetime_str = format!("{}T{}", date_str, time_str);

        match holyc.solar_calc(self.lat, self.lon, &datetime_str) {
            Ok(ref map) => {
                self.sunrise = map.get("sunrise").cloned();
                self.sunset = map.get("sunset").cloned();
                self.solar_noon = map.get("solar_noon").cloned();
                self.day_length = map
                    .get("day_length")
                    .and_then(|v: &String| v.parse::<f64>().ok());
                self.equation_of_time = map
                    .get("equation_of_time")
                    .and_then(|v: &String| v.parse::<f64>().ok());
            }
            Err(_) => {
                self.sunrise = None;
                self.sunset = None;
                self.solar_noon = None;
                self.day_length = None;
                self.equation_of_time = None;
            }
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        lines.push(Line::from(Span::raw(format!(
            "Solar data at lat={}, lon={}:",
            self.lat, self.lon
        ))));

        match &self.sunrise {
            Some(s) => lines.push(Line::from(Span::raw(format!("  Sunrise:       {}", s)))),
            None => lines.push(Line::from(Span::raw("  Sunrise:       (unavailable)"))),
        }
        match &self.sunset {
            Some(s) => lines.push(Line::from(Span::raw(format!("  Sunset:        {}", s)))),
            None => lines.push(Line::from(Span::raw("  Sunset:        (unavailable)"))),
        }
        match &self.solar_noon {
            Some(s) => lines.push(Line::from(Span::raw(format!("  Solar noon:    {}", s)))),
            None => lines.push(Line::from(Span::raw("  Solar noon:    (unavailable)"))),
        }
        match self.day_length {
            Some(d) => {
                let hours = d.trunc() as u64;
                let minutes = ((d - hours as f64) * 60.0).round() as u64;
                lines.push(Line::from(Span::raw(format!(
                    "  Day length:    {}h {:02}m",
                    hours, minutes
                ))));
            }
            None => lines.push(Line::from(Span::raw("  Day length:    (unavailable)"))),
        }
        match self.equation_of_time {
            Some(e) => lines.push(Line::from(Span::raw(format!(
                "  Equation of time: {:.2} min",
                e
            )))),
            None => lines.push(Line::from(Span::raw(
                "  Equation of time: (unavailable)",
            ))),
        }

        lines
    }
}
