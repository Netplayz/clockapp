use chrono::{Local, Timelike};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

pub struct AnalogClockState {
    pub rad: f64,
}

impl AnalogClockState {
    pub fn new() -> Self {
        Self { rad: 5.0 }
    }

    pub fn update(&mut self) {}

    pub fn render(&self) -> Vec<Line<'static>> {
        let now = Local::now();
        let hour = now.hour12().1 as f64 + now.minute() as f64 / 60.0;
        let minute = now.minute() as f64 + now.second() as f64 / 60.0;

        let rows = 13;
        let cols = 13;
        let cx = 6.0;
        let cy = 6.0;
        let r = self.rad;

        let mut grid = vec![vec![' '; cols]; rows];

        for h in 0..12 {
            let angle_deg = h as f64 * 30.0;
            let rad = angle_deg.to_radians();
            let x = (cx + r * rad.sin()).round() as i32;
            let y = (cy - r * rad.cos()).round() as i32;
            if x >= 0 && x < cols as i32 && y >= 0 && y < rows as i32 {
                grid[y as usize][x as usize] = '·';
            }
        }

        grid[0][5] = '1';
        grid[0][6] = '2';
        grid[6][12] = '3';
        grid[12][6] = '6';
        grid[6][0] = '9';

        let h_angle = (hour % 12.0) * 30.0 * std::f64::consts::PI / 180.0;
        let h_len = 3.0;
        for i in 1..=6 {
            let t = i as f64 / 6.0;
            let x = (cx + h_len * t * h_angle.sin()).round() as i32;
            let y = (cy - h_len * t * h_angle.cos()).round() as i32;
            if x >= 0 && x < cols as i32 && y >= 0 && y < rows as i32 {
                grid[y as usize][x as usize] = '▓';
            }
        }

        let m_angle = minute * 6.0 * std::f64::consts::PI / 180.0;
        let m_len = 4.0;
        for i in 1..=8 {
            let t = i as f64 / 8.0;
            let x = (cx + m_len * t * m_angle.sin()).round() as i32;
            let y = (cy - m_len * t * m_angle.cos()).round() as i32;
            if x >= 0 && x < cols as i32 && y >= 0 && y < rows as i32 {
                grid[y as usize][x as usize] = '▒';
            }
        }

        grid[6][6] = '◉';

        let style = Style::new().fg(Color::Cyan);
        grid.into_iter()
            .map(|row| Line::from(Span::styled(row.into_iter().collect::<String>(), style)))
            .collect()
    }
}
