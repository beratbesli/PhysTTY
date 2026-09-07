mod physics;
mod renderer;

use std::{
    env,
    error::Error,
    io::{self, Stdout, Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{
        self, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};

use physics::{Bounds, PhysicsWorld};
use renderer::{field_bounds, render};

const FIXED_STEP: f32 = 1.0 / 120.0;
const MAX_STEPS_PER_FRAME: usize = 8;
const INPUT_POLL: Duration = Duration::from_millis(8);
const RENDER_INTERVAL: Duration = Duration::from_millis(33);

fn main() -> Result<(), Box<dyn Error>> {
    run()
}

fn run() -> Result<(), Box<dyn Error>> {
    let _terminal = TerminalGuard::enter()?;
    let mut stdout = stdout();
    let ascii = env::var_os("PHYSTTY_ASCII").is_some();
    let mut app = App::new();
    let mut accumulator = 0.0_f32;
    let mut previous = Instant::now();
    let mut last_render = Instant::now() - RENDER_INTERVAL;
    let mut last_size = None;

    loop {
        while event::poll(INPUT_POLL)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if app.should_quit {
            break;
        }

        let now = Instant::now();
        let elapsed = now.duration_since(previous).as_secs_f32().min(0.25);
        previous = now;
        accumulator += elapsed;

        let size = terminal::size()?;
        let bounds = field_bounds(size);
        if last_size != Some(size) {
            app.world.resize(bounds);
            last_size = Some(size);
        }

        if !app.paused {
            let mut steps = 0;
            while accumulator >= FIXED_STEP && steps < MAX_STEPS_PER_FRAME {
                app.world.step(FIXED_STEP);
                accumulator -= FIXED_STEP;
                steps += 1;
            }
            if steps == MAX_STEPS_PER_FRAME {
                accumulator = 0.0;
            }
        } else {
            accumulator = 0.0;
        }

        if last_render.elapsed() >= RENDER_INTERVAL {
            render(&mut stdout, size, &app.world, app.paused, ascii)?;
            stdout.flush()?;
            last_render = Instant::now();
        }
    }

    Ok(())
}

struct App {
    world: PhysicsWorld,
    paused: bool,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let mut world = PhysicsWorld::new(Bounds::new(78.0, 40.0));
        world.reset();
        Self {
            world,
            paused: false,
            should_quit: false,
        }
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => self.should_quit = true,
            KeyCode::Char(' ') => self.world.spawn_ball(),
            KeyCode::Char('g') | KeyCode::Char('G') => self.world.toggle_gravity(),
            KeyCode::Char('p') | KeyCode::Char('P') => self.paused = !self.paused,
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.world.reset();
                self.paused = false;
            }
            KeyCode::Char('c') | KeyCode::Char('C') => self.world.clear_dynamic(),
            _ => {}
        }
    }
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        enable_raw_mode()?;
        if let Err(error) = execute!(stdout(), EnterAlternateScreen, Hide) {
            let _ = disable_raw_mode();
            return Err(error);
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout: Stdout = stdout();
        let _ = execute!(stdout, Show, LeaveAlternateScreen);
    }
}
