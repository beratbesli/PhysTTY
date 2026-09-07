# PhysTTY

PhysTTY is a lightweight real-time 2D physics sandbox rendered entirely inside your terminal.

## What is PhysTTY?

PhysTTY turns a terminal window into a small physics playground. Balls fall under gravity, bounce off the world boundaries, and collide with one another. The simulation is implemented directly in Rust rather than delegated to a full physics engine.

## Demo

```text
┌──────────────────────────────────────────────────────────┐
│ PhysTTY | Bodies: 3 | Gravity: ON | Running              │
│                         ●                                │
│                 ●                         ●              │
│                                                          │
│                                                          │
│ SPACE spawn | G gravity | P pause | R reset | C clear    │
└──────────────────────────────────────────────────────────┘
```

## Features

- Fixed-timestep gravity and velocity integration
- Boundary collisions with restitution and penetration correction
- Circle-to-circle collisions
- Runtime spawning, pause, reset, clear, and gravity controls
- Alternate-screen rendering with hidden cursor and terminal cleanup
- Resize-aware simulation area and compact HUD
- Optional ASCII rendering with `PHYSTTY_ASCII=1`

## Installation and build

Install [Rust](https://www.rust-lang.org/tools/install), then run:

```bash
git clone https://github.com/beratbesli/PhysTTY.git
cd PhysTTY
cargo run --release
```

The terminal should be at least 24 columns by 8 rows. Press `Q` or `Esc` to quit.

## Controls

| Key | Action |
| --- | --- |
| `Space` | Spawn a ball |
| `G` | Toggle gravity |
| `P` | Pause or resume |
| `R` | Reset the initial scene |
| `C` | Clear dynamic balls |
| `Q` / `Esc` | Quit |

## Development

Run the project checks with:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Project philosophy

PhysTTY favors a small, explicit architecture over a general-purpose engine. The simulation uses float-based positions, a fixed physics timestep, simple circles, and terminal-friendly rendering. Every feature should make the sandbox easier to understand or more pleasant to use.

## Roadmap

- More static obstacles while keeping collision rules simple
- Optional color themes
- Small deterministic scenario presets

Features outside the core terminal sandbox are intentionally out of scope for the first release.
