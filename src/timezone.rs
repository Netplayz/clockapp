use chrono::Offset;
use chrono::Utc;
use chrono_tz::Tz;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

pub struct TimezoneConverterState {
    pub source_tz: String,
    pub target_tz: String,
    pub source_time: String,
}

impl TimezoneConverterState {
    pub fn new() -> Self {
        Self {
            source_tz: "America/New_York".into(),
            target_tz: "Europe/London".into(),
            source_time: String::new(),
        }
    }

    pub fn render(&self) -> Vec<Line<'static>> {
        let now_utc = Utc::now();
        let mut lines = Vec::new();

        lines.push(Line::from(Span::styled(
            "Timezone Converter",
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));

        lines.push(Line::from(Span::raw("")));

        let src_tz: Option<Tz> = self.source_tz.parse().ok();
        let dst_tz: Option<Tz> = self.target_tz.parse().ok();

        if let Some(ref tz) = src_tz {
            let local = now_utc.with_timezone(tz);
            let offset = local.offset().fix().local_minus_utc();
            let oh = offset / 3600;
            let os = if offset >= 0 { '+' } else { '-' };
            lines.push(Line::from(vec![
                Span::styled("Source:      ", Style::new().fg(Color::White)),
                Span::styled(
                    format!("{} ({})", self.source_tz, local.format("%H:%M:%S")),
                    Style::new().fg(Color::Green),
                ),
                Span::styled(
                    format!("  UTC{}{:02}:00", os, oh.abs()),
                    Style::new().fg(Color::Yellow),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("Source:      ", Style::new().fg(Color::White)),
                Span::styled(
                    format!("{} (invalid)", self.source_tz),
                    Style::new().fg(Color::Red),
                ),
            ]));
        }

        if let Some(ref tz) = dst_tz {
            let local = now_utc.with_timezone(tz);
            let offset = local.offset().fix().local_minus_utc();
            let oh = offset / 3600;
            let os = if offset >= 0 { '+' } else { '-' };
            lines.push(Line::from(vec![
                Span::styled("Target:      ", Style::new().fg(Color::White)),
                Span::styled(
                    format!("{} ({})", self.target_tz, local.format("%H:%M:%S")),
                    Style::new().fg(Color::Green),
                ),
                Span::styled(
                    format!("  UTC{}{:02}:00", os, oh.abs()),
                    Style::new().fg(Color::Yellow),
                ),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled("Target:      ", Style::new().fg(Color::White)),
                Span::styled(
                    format!("{} (invalid)", self.target_tz),
                    Style::new().fg(Color::Red),
                ),
            ]));
        }

        lines.push(Line::from(Span::raw("")));

        if let (Some(ref src), Some(ref dst)) = (src_tz, dst_tz) {
            let src_local = now_utc.with_timezone(src);
            let dst_local = now_utc.with_timezone(dst);
            let diff_secs =
                dst_local.offset().fix().local_minus_utc() - src_local.offset().fix().local_minus_utc();
            let diff_hours = diff_secs as f64 / 3600.0;
            lines.push(Line::from(Span::styled(
                format!(
                    "{} in {} is {} in {}",
                    src_local.format("%H:%M"),
                    self.source_tz,
                    dst_local.format("%H:%M"),
                    self.target_tz,
                ),
                Style::new().fg(Color::Cyan),
            )));
            lines.push(Line::from(Span::styled(
                format!("Time difference: {:.1} hours", diff_hours.abs()),
                Style::new().fg(Color::Yellow),
            )));
        }

        lines
    }
}
