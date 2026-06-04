# clockapp

A feature-rich terminal clock application built with Rust, Brainfuck, and HolyC.

## Features

### 12 TUI Modules
| Module | Description |
|---|---|
| **Digital Clock** | 12h, 24h, ISO 8601, RFC 2822, Unix epoch formats |
| **Analog Clock** | ASCII art clock face with hour/minute/second hands |
| **Stopwatch** | Lap recording with millisecond precision |
| **Timer** | Concurrent timers with named presets and progress bars |
| **World Clock** | 10 preset cities with DST-aware timezone display |
| **Timezone Converter** | Dual timezone comparison |
| **Countdown** | Target date countdown with progress bar |
| **Alarm Manager** | Persistent recurring alarms with enable/disable |
| **Solar/Sundial** | Sunrise, sunset, solar noon, equation of time |
| **Astronomy** | Julian Day, moon phase, seasons |
| **Calendar** | Month grid with week numbers, day info, event slots |
| **Time Math** | Date difference, date addition, age, week-of-year |

### Architecture
- **Rust** — main orchestrator with ratatui TUI, embedded Brainfuck interpreter, HolyC subprocess manager
- **Brainfuck** — 4 subcomponents embedded at compile time via `include_str!`, interpreted at runtime
- **HolyC** — 4 compiled ELF binaries called via subprocess with stdin/stdout protocol

## Dependencies
- Rust 2021 edition
- Dependencies: ratatui 0.29, crossterm 0.28, chrono 0.4, chrono-tz 0.10, tokio 1, serde 1, serde_json 1, dirs 6
- HolyC-lang compiler (`hcc`) optional — only needed to recompile HolyC subcomponents

## Build & Run

```sh
# Build Rust application
cargo build --release

# Build HolyC subcomponents (optional)
./build_holyc.sh

# Run
cargo run
```

### Controls
| Key | Action |
|---|---|
| `Tab` / `Right` | Next tab |
| `Shift+Tab` / `Left` | Previous tab |
| `q` / `Esc` | Quit |

## HolyC Toolchain

HolyC programs are pre-compiled to ELF binaries in `holyc/`. To rebuild:

```sh
# Requires hcc in PATH
./build_holyc.sh
```

The HolyC subcomponents provide:
- `solar_calc` — solar position, sunrise/sunset, equation of time
- `astronomy` — Julian Day, moon phase, season calculation
- `alarm_server` — file-based alarm watcher
- `date_math` — date difference, addition, week-of-year, age

## Brainfuck Programs

BF programs in `bf/` are embedded at compile time:
- `ticktock.bf` — alternating tick/tock pattern generator
- `binary_clock.bf` — dot-count visual clock row renderer
- `prime_minute.bf` — primality checker for 0–59
- `ascii_digits.bf` — big digit renderer from ASCII input

## License

MIT
