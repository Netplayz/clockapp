use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    pub label: String,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub recurring: bool,
    pub days_of_week: Vec<u8>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimerPreset {
    pub label: String,
    pub duration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldCity {
    pub name: String,
    pub country: String,
    pub timezone: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub alarms: Vec<AlarmConfig>,
    pub timer_presets: Vec<TimerPreset>,
    pub world_cities: Vec<WorldCity>,
    pub last_tab: Option<String>,
    pub twenty_four_hour: bool,
    pub show_seconds: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            alarms: Vec::new(),
            timer_presets: Vec::new(),
            world_cities: vec![
                WorldCity {
                    name: "New York".into(),
                    country: "US".into(),
                    timezone: "America/New_York".into(),
                    lat: 40.7128,
                    lon: -74.0060,
                },
                WorldCity {
                    name: "London".into(),
                    country: "UK".into(),
                    timezone: "Europe/London".into(),
                    lat: 51.5074,
                    lon: -0.1278,
                },
                WorldCity {
                    name: "Tokyo".into(),
                    country: "Japan".into(),
                    timezone: "Asia/Tokyo".into(),
                    lat: 35.6762,
                    lon: 139.6503,
                },
                WorldCity {
                    name: "Sydney".into(),
                    country: "Australia".into(),
                    timezone: "Australia/Sydney".into(),
                    lat: -33.8688,
                    lon: 151.2093,
                },
                WorldCity {
                    name: "Dubai".into(),
                    country: "UAE".into(),
                    timezone: "Asia/Dubai".into(),
                    lat: 25.2048,
                    lon: 55.2708,
                },
                WorldCity {
                    name: "Paris".into(),
                    country: "France".into(),
                    timezone: "Europe/Paris".into(),
                    lat: 48.8566,
                    lon: 2.3522,
                },
                WorldCity {
                    name: "Moscow".into(),
                    country: "Russia".into(),
                    timezone: "Europe/Moscow".into(),
                    lat: 55.7558,
                    lon: 37.6173,
                },
                WorldCity {
                    name: "Singapore".into(),
                    country: "Singapore".into(),
                    timezone: "Asia/Singapore".into(),
                    lat: 1.3521,
                    lon: 103.8198,
                },
                WorldCity {
                    name: "San Francisco".into(),
                    country: "US".into(),
                    timezone: "America/Los_Angeles".into(),
                    lat: 37.7749,
                    lon: -122.4194,
                },
                WorldCity {
                    name: "Berlin".into(),
                    country: "Germany".into(),
                    timezone: "Europe/Berlin".into(),
                    lat: 52.5200,
                    lon: 13.4050,
                },
            ],
            last_tab: None,
            twenty_four_hour: true,
            show_seconds: true,
        }
    }
}

pub struct Storage {
    path: PathBuf,
    config: AppConfig,
}

impl Storage {
    pub fn new() -> Self {
        let config_dir = dirs::home_dir()
            .unwrap_or_default()
            .join(".clockapp");
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }
        let path = config_dir.join("config.json");
        let config = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|data| serde_json::from_str(&data).ok())
                .unwrap_or_default()
        } else {
            AppConfig::default()
        };
        Self { path, config }
    }

    pub fn load(&mut self) {
        if self.path.exists() {
            if let Ok(data) = fs::read_to_string(&self.path) {
                if let Ok(cfg) = serde_json::from_str(&data) {
                    self.config = cfg;
                }
            }
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let data = serde_json::to_string_pretty(&self.config)
            .map_err(|e| e.to_string())?;
        fs::write(&self.path, data).map_err(|e| e.to_string())
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut AppConfig {
        &mut self.config
    }

    pub fn alarms(&self) -> &Vec<AlarmConfig> {
        &self.config.alarms
    }

    pub fn alarms_mut(&mut self) -> &mut Vec<AlarmConfig> {
        &mut self.config.alarms
    }

    pub fn timer_presets(&self) -> &Vec<TimerPreset> {
        &self.config.timer_presets
    }

    pub fn world_cities(&self) -> &Vec<WorldCity> {
        &self.config.world_cities
    }
}
