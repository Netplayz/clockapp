# clockapp

> _"An idiot admires complexity, a genius admires simplicity."_ — Terry A. Davis
>
> _Well Terry, we not the geniuses in this scenario._

> _Because one language wasn't sufficiently overengineered for telling time._

A terminal clock with no good reason to be written in three programming languages, yet here we are.

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

| Language | Role | Why |
|---|---|---|
| **Rust** | Main orchestrator, TUI, BF interpreter, HolyC subprocess manager | The sensible choice |
| **Brainfuck** | 4 embedded subcomponents via `include_str!`, interpreted at runtime | The sensible choice's rebellious phase |
| **HolyC** | 4 compiled ELF binaries called via subprocess | The sensible choice's midlife crisis |

Why limit yourself to one bad decision when you can make three?

## Dependencies

- Rust 2021 edition
- Dependencies: ratatui 0.29, crossterm 0.28, chrono 0.4, chrono-tz 0.10, tokio 1, serde 1, serde_json 1, dirs 6
- HolyC-lang compiler (`hcc`) optional — only needed if you want to recompile the HolyC subcomponents for some reason

## Build & Run

```sh
# Build Rust application
cargo build --release

# Build HolyC subcomponents (optional — includes free existential dread)
./build_holyc.sh

# Run
cargo run
```

### Controls
| Key | Action |
|---|---|
| `Tab` / `Right` | Next tab |
| `Shift+Tab` / `Left` | Previous tab |
| `h` | Toggle 12h/24h (Digital Clock tab) |
| `s` | Toggle seconds (Digital Clock tab) / Start/stop stopwatch (elsewhere) |
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

Yes, we wrote an alarm server and solar calculator in HolyC. No, we don't regret it. (We do, actually.)

## Brainfuck Programs

BF programs in `bf/` are embedded at compile time:
- `ticktock.bf` — alternating tick/tock pattern generator
- `binary_clock.bf` — dot-count visual clock row renderer
- `prime_minute.bf` — primality checker for 0–59 (not actually faster than just hardcoding the primes, but way more entertaining)
- `ascii_digits.bf` — big digit renderer from ASCII input

These programs run on an interpreter we wrote ourselves, because importing one would have been too easy.

## Why Did You Do This

- The original author was left unsupervised with a terminal.
- Rust is great for things that should work correctly; Brainfuck is great for things that shouldn't work at all; HolyC is great for things you want to regret.
- We believe clocks should be complicated. It builds character.

## Is This Production Ready

Absolutely not.

## Will You Maintain This

Probably not.

## License

MIT — because even bad ideas deserve legal protection.
