use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use ratatui::Frame;

use clockapp::alarm::AlarmState;
use clockapp::astronomy::AstronomyState;
use clockapp::calendar::CalendarState;
use clockapp::clock;
use clockapp::clock::world::WorldCityDisplay;
use clockapp::countdown::CountdownState;
use clockapp::ipc;
use clockapp::storage::Storage;
use clockapp::stopwatch::StopwatchState;
use clockapp::timemath::TimeMathState;
use clockapp::timer::TimerState;
use clockapp::timezone::TimezoneConverterState;
use chrono_tz::Tz;
use crossterm::event::KeyCode;

#[allow(dead_code)]
pub enum Tab {
    DigitalClock,
    AnalogClock,
    Stopwatch,
    Timer,
    WorldClock,
    TimezoneConverter,
    Countdown,
    Alarm,
    Solar,
    Astronomy,
    Calendar,
    TimeMath,
}

pub struct App {
    pub current_tab: usize,
    pub tab_labels: Vec<&'static str>,
    pub digital_clock: clock::digital::DigitalClockState,
    pub analog_clock: clock::analog::AnalogClockState,
    pub stopwatch: StopwatchState,
    pub timer: TimerState,
    pub world_clock: clock::world::WorldCityState,
    pub timezone_converter: TimezoneConverterState,
    pub countdown: CountdownState,
    pub alarm: AlarmState,
    pub solar: clock::solar::SolarState,
    pub astronomy: AstronomyState,
    pub calendar: CalendarState,
    pub time_math: TimeMathState,
    #[allow(dead_code)]
    pub storage: Storage,
    pub holyc: Option<ipc::holyc::HolyCManager>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let storage = Storage::new();

        let world_cities: Vec<WorldCityDisplay> = storage
            .world_cities()
            .iter()
            .filter_map(|wc| {
                let tz: Tz = wc.timezone.parse().ok()?;
                Some(WorldCityDisplay {
                    name: wc.name.clone(),
                    timezone: tz,
                })
            })
            .collect();

        let holyc = Some(ipc::holyc::HolyCManager::new());

        Self {
            current_tab: 0,
            tab_labels: vec![
                "Digital",
                "Analog",
                "Stopwatch",
                "Timer",
                "World",
                "TZ Conv",
                "Countdown",
                "Alarm",
                "Solar",
                "Astronomy",
                "Calendar",
                "TimeMath",
            ],
            digital_clock: clock::digital::DigitalClockState::new(),
            analog_clock: clock::analog::AnalogClockState::new(),
            stopwatch: StopwatchState::new(),
            timer: TimerState::new(),
            world_clock: clock::world::WorldCityState::new(world_cities),
            timezone_converter: TimezoneConverterState::new(),
            countdown: CountdownState::new(),
            alarm: AlarmState::new(),
            solar: clock::solar::SolarState::new(),
            astronomy: AstronomyState::new(),
            calendar: CalendarState::new(),
            time_math: TimeMathState::new(),
            storage,
            holyc,
            should_quit: false,
        }
    }

    pub fn update(&mut self) {
        self.digital_clock.update();
        self.analog_clock.update();
        self.stopwatch.update();
        self.timer.update();
        self.world_clock.update();
        self.countdown.update();
        self.alarm.check_alarms();
        self.calendar.update();

        if let Some(ref holyc) = self.holyc {
            self.solar.calculate(holyc);
            self.astronomy.calculate(holyc);
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab | KeyCode::Right => {
                self.current_tab = (self.current_tab + 1) % self.tab_labels.len();
            }
            KeyCode::Left | KeyCode::BackTab => {
                self.current_tab = if self.current_tab == 0 {
                    self.tab_labels.len() - 1
                } else {
                    self.current_tab - 1
                };
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('s') => {
                if self.stopwatch.running {
                    self.stopwatch.stop();
                } else {
                    self.stopwatch.start();
                }
            }
            KeyCode::Char('l') => {
                self.stopwatch.lap();
            }
            KeyCode::Char('r') => {
                self.stopwatch.reset();
            }
            KeyCode::Char('a') if self.current_tab == 3 => {
                self.timer.add_timer("Timer".to_string(), 300);
            }
            _ => {}
        }
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let tab_titles: Vec<Line> = self.tab_labels.iter().map(|s| Line::from(*s)).collect();
        let tabs = Tabs::new(tab_titles)
            .select(self.current_tab)
            .block(Block::default().borders(Borders::ALL).title("ClockApp"));
        f.render_widget(tabs, layout[0]);

        let lines = match self.current_tab {
            0 => self.digital_clock.render(),
            1 => self.analog_clock.render(),
            2 => self.stopwatch.render(),
            3 => self.timer.render(),
            4 => self.world_clock.render(),
            5 => self.timezone_converter.render(),
            6 => self.countdown.render(),
            7 => self.alarm.render(),
            8 => self.solar.render(),
            9 => self.astronomy.render(),
            10 => self.calendar.render(),
            11 => self.time_math.render(),
            _ => vec![Line::from(Span::raw("Unknown tab"))],
        };

        let paragraph = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(self.tab_labels[self.current_tab]),
        );
        f.render_widget(paragraph, layout[1]);
    }
}
