
use std::io;
use std::io::Write;
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

use crossterm::event::poll;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;


fn main() {
            // SETUP //
    let mut out = io::stdout();
    let (w, h) = terminal::size().unwrap();
    let (c_x, c_y) = (w as i16/ 2, h as i16 / 2);
    let mut v: (i16, i16) = (1, 0);
    let mut sym = '>';

    enable_raw_mode().unwrap();
    execute!(out, EnterAlternateScreen).unwrap();
    execute!(out, cursor::Hide).unwrap();

        // WALLS //
    for x in 0..w {
        queue!(out, MoveTo(x, 0), Print('+'), MoveTo(x, h - 1), Print('+')).unwrap();
    }
    for y in 0..h {
        queue!(out, MoveTo(0, y), Print('+'), MoveTo(w - 1, y), Print('+')).unwrap();
    }
    out.flush().unwrap();
    

        // INIT SNAKE //
    let mut deque: VecDeque<(i16, i16)> = VecDeque::new();
    for d in 0..30 {
        deque.push_back((c_x + d, c_y));
        execute!(out, MoveTo((c_x + d) as u16, c_y as u16), Print('#')).unwrap();
    }

    loop {
                // CONTROLS //
        if poll(Duration::from_millis(200)).unwrap()
                && let Event::Key(event) = read().unwrap() {
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
            if hx <= 1 || hx >= w as i16 - 2
                    || hy <= 1 || hy >= h as i16 - 2
                    || deque.contains(&(hx, hy)) {
                break;
            }
            deque.push_back((hx, hy));
        }

                // SNAKE MOVEMENT //
        if let Some((x, y)) = deque.pop_front() {
            execute!(out, MoveTo(x as u16, y as u16), Print(' ')).unwrap();
        }
        if let Some(&(x, y)) = deque.back() {
            execute!(out, MoveTo(x as u16, y as u16), Print('#')).unwrap();
            let n = ((x + v.0), (y + v.1));
            execute!(out, MoveTo(n.0 as u16, n.1 as u16), Print(sym)).unwrap();
            deque.push_back(n);
        }
    }

            // CLEANUP //
    execute!(out, LeaveAlternateScreen).unwrap();
    execute!(out, cursor::Show).unwrap();
    disable_raw_mode().unwrap();
}

