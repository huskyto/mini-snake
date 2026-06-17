
use std::io;
use std::io::Write;
use std::error::Error;
use std::time::Duration;
use std::collections::VecDeque;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;

use crossterm::queue;
use crossterm::execute;

use rand::random_range;
use crossterm::event::poll;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;


fn main() {
    let _ = standard_snake();
}


fn standard_snake() -> Result<(), Box<dyn Error>> {
            // SETUP //
    let mut out = io::stdout();
    let (w, h) = terminal::size()?;
    let (c_x, c_y) = (w as i16/ 2, h as i16 / 2);
    let mut v: (i16, i16) = (1, 0);
    let mut apple = (random_range(1..w -1 ), random_range(1..h - 1));
    let mut sym = '>';
    let mut delay = 200;

    enable_raw_mode()?;
    execute!(out, EnterAlternateScreen)?;
    execute!(out, cursor::Hide)?;

        // WALLS //
    for x in 0..w {
        queue!(out, MoveTo(x, 0), Print('+'), MoveTo(x, h - 1), Print('+'))?;
    }
    for y in 0..h {
        queue!(out, MoveTo(0, y), Print('+'), MoveTo(w - 1, y), Print('+'))?;
    }
    out.flush()?;

        // INIT SNAKE //
    let mut deque: VecDeque<(i16, i16)> = VecDeque::new();
    for d in 0..3 {
        deque.push_back((c_x + d, c_y));
        execute!(out, MoveTo((c_x + d) as u16, c_y as u16), Print('#'))?;
    }

        // APPLE //
    execute!(out, MoveTo(apple.0, apple.1), Print('O'))?;

    loop {
                // CONTROLS //
        if poll(Duration::from_millis(delay))?
                && let Event::Key(event) = read()? {
            match event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('w') if sym != 'v' => (sym, v) = ('^', ( 0,-1)),
                KeyCode::Char('a') if sym != '>' => (sym, v) = ('<', (-1, 0)),
                KeyCode::Char('s') if sym != '^' => (sym, v) = ('v', ( 0, 1)),
                KeyCode::Char('d') if sym != '<' => (sym, v) = ('>', ( 1, 0)),
                _ => {}
            }
        }

                // LOSE CONDITIONS //
        
        if let Some((hx, hy)) = deque.pop_back() {
            if hx <= 0 || hx >= w as i16 - 1
                    || hy <= 0 || hy >= h as i16 - 1
                    || deque.contains(&(hx, hy)) {
                break;
            }
            deque.push_back((hx, hy));
        }

                // EAT APPLE //
        if let Some(&(hx, hy)) = deque.back()
                && hx == apple.0 as i16 && hy == apple.1 as i16
                && let Some(&(x, y)) = deque.front() {
            deque.push_front((x, y));
            loop {
                apple = (random_range(1..w - 1), random_range(1..h - 1));
                if !deque.contains(&(apple.0 as i16, apple.1 as i16)) { break }
            }
            execute!(out, MoveTo(apple.0, apple.1), Print('O'))?;
            delay = ((delay as f32 * 0.90) as u64).max(50);
        }

                // SNAKE MOVEMENT //
        if let Some((x, y)) = deque.pop_front() {
            execute!(out, MoveTo(x as u16, y as u16), Print(' '))?;
        }
        if let Some(&(x, y)) = deque.back() {
            execute!(out, MoveTo(x as u16, y as u16), Print('#'))?;
            let n = ((x + v.0), (y + v.1));
            execute!(out, MoveTo(n.0 as u16, n.1 as u16), Print(sym))?;
            deque.push_back(n);
        }
    }

            // CLEANUP //
    execute!(out, LeaveAlternateScreen)?;
    execute!(out, cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}
