use std::io::{self, Stdout};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::Print,
    terminal::{Clear, ClearType},
};

use crate::physics::{Bounds, PhysicsWorld};

const CELL_ASPECT: f32 = 2.0;
const MIN_COLUMNS: u16 = 24;
const MIN_ROWS: u16 = 8;

pub fn field_bounds(size: (u16, u16)) -> Bounds {
    let columns = size.0.saturating_sub(2).max(2);
    let rows = size.1.saturating_sub(4).max(2);
    Bounds::new(f32::from(columns), f32::from(rows) * CELL_ASPECT)
}

pub fn render(
    stdout: &mut Stdout,
    size: (u16, u16),
    world: &PhysicsWorld,
    paused: bool,
    ascii: bool,
) -> io::Result<()> {
    if size.0 < MIN_COLUMNS || size.1 < MIN_ROWS {
        let message = "PhysTTY needs a larger terminal window.";
        queue!(
            stdout,
            MoveTo(0, 0),
            Clear(ClearType::All),
            Print(message),
            Clear(ClearType::FromCursorDown)
        )?;
        return Ok(());
    }

    let width = usize::from(size.0);
    let field_rows = usize::from(size.1.saturating_sub(4));
    let border = if ascii { '-' } else { '─' };
    let corner = if ascii { '+' } else { '┼' };
    let side = if ascii { '|' } else { '│' };
    let ball = if ascii { 'o' } else { '●' };
    let mut lines = Vec::with_capacity(usize::from(size.1));
    lines.push(format!(
        "{}{}{}",
        corner,
        border.to_string().repeat(width - 2),
        corner
    ));
    lines.push(frame_line(
        &format!(
            " PhysTTY | Bodies: {} | Gravity: {} | {} ",
            world.bodies.len(),
            if world.gravity_enabled { "ON" } else { "OFF" },
            if paused { "Paused" } else { "Running" }
        ),
        width,
        side,
    ));

    let mut body_cells = vec![vec![' '; width - 2]; field_rows];
    for body in &world.bodies {
        let x = body.position.x.round() as isize;
        let y = (body.position.y / CELL_ASPECT).round() as isize;
        if let (Ok(x), Ok(y)) = (usize::try_from(x), usize::try_from(y))
            && let Some(row) = body_cells.get_mut(y)
            && let Some(cell) = row.get_mut(x)
        {
            *cell = ball;
        }
    }
    for row in body_cells {
        lines.push(format!(
            "{}{}{}",
            side,
            row.into_iter().collect::<String>(),
            side
        ));
    }

    lines.push(frame_line(
        " SPACE spawn | G gravity | P pause | R reset | C clear | Q quit ",
        width,
        side,
    ));
    lines.push(format!(
        "{}{}{}",
        corner,
        border.to_string().repeat(width - 2),
        corner
    ));

    let frame = lines.join("\r\n");
    queue!(
        stdout,
        MoveTo(0, 0),
        Print(frame),
        Clear(ClearType::FromCursorDown)
    )?;
    Ok(())
}

fn frame_line(content: &str, width: usize, side: char) -> String {
    let inner_width = width - 2;
    let content: String = content.chars().take(inner_width).collect();
    format!(
        "{}{}{}{}",
        side,
        content,
        " ".repeat(inner_width - content.chars().count()),
        side
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_bounds_reserve_hud_and_borders() {
        let bounds = field_bounds((82, 30));

        assert_eq!(bounds.width, 80.0);
        assert_eq!(bounds.height, 52.0);
    }

    #[test]
    fn frame_line_is_exactly_terminal_width() {
        let line = frame_line("hello", 12, '|');

        assert_eq!(line.chars().count(), 12);
        assert_eq!(line, "|hello     |");
    }
}
