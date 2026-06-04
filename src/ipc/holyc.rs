use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

pub struct HolyCManager {
    binaries_dir: PathBuf,
}

impl HolyCManager {
    pub fn new() -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let binaries_dir = root.join("holyc");
        HolyCManager { binaries_dir }
    }

    pub fn solar_calc(&self, lat: f64, lon: f64, datetime: &str) -> Result<HashMap<String, String>, String> {
        let output = run_holyc(
            &self.binaries_dir.join("solar_calc"),
            &[&lat.to_string(), &lon.to_string(), datetime],
        )?;
        Ok(parse_kv_output(&output))
    }

    pub fn astronomy(&self, date: &str) -> Result<HashMap<String, String>, String> {
        let output = run_holyc(&self.binaries_dir.join("astronomy"), &[date])?;
        Ok(parse_kv_output(&output))
    }

    pub fn alarm_check(&self, alarm_file: &str) -> Result<Vec<String>, String> {
        let output = run_holyc(&self.binaries_dir.join("alarm_server"), &["CHECK", alarm_file])?;
        let labels: Vec<String> = output
            .lines()
            .filter(|line| line.starts_with("ALARM:"))
            .map(|line| line[6..].to_string())
            .collect();
        Ok(labels)
    }

    pub fn date_math_diff(&self, date1: &str, date2: &str) -> Result<HashMap<String, String>, String> {
        let output = run_holyc(&self.binaries_dir.join("date_math"), &["diff", date1, date2])?;
        Ok(parse_kv_output(&output))
    }

    pub fn date_math_add(&self, date: &str, days: i64) -> Result<String, String> {
        let output = run_holyc(&self.binaries_dir.join("date_math"), &["add", date, &days.to_string()])?;
        Ok(output.trim().to_string())
    }

    pub fn date_math_weekof(&self, date: &str) -> Result<HashMap<String, String>, String> {
        let output = run_holyc(&self.binaries_dir.join("date_math"), &["weekof", date])?;
        Ok(parse_kv_output(&output))
    }

    pub fn date_math_age(&self, date: &str) -> Result<String, String> {
        let output = run_holyc(&self.binaries_dir.join("date_math"), &["age", date])?;
        Ok(output.trim().to_string())
    }
}

fn run_holyc(bin_path: &PathBuf, args: &[&str]) -> Result<String, String> {
    if !bin_path.exists() {
        return Err(format!("HolyC binary not found: {}", bin_path.display()));
    }
    let output = Command::new(bin_path)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute {}: {}", bin_path.display(), e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{} exited with error: {}", bin_path.display(), stderr.trim()));
    }
    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 output from {}: {}", bin_path.display(), e))
}

fn parse_kv_output(output: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in output.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos + 1..].trim().to_string();
            map.insert(key, value);
        }
    }
    map
}
