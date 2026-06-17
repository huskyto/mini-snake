# Mini Snake

A terminal Snake game in pure Rust, written twice over.

The goal of this little project is to build the shortest *and* the simplest
possible implementations of Snake. The two versions are functionally identical;
they just optimize for different things:

- **`standard_snake`**: readable and commented, the "simple" version.
- **`min_snake`**: the same game squeezed into as little code as reasonable.

No obfuscation, no cramming everything onto one line. The minimal version stays
intelligible. Clever tricks are fair game, golfing for its own sake is not.

## Running

```sh
cargo run
```

By default `main.rs` runs both versions back to back. Comment out whichever you
don't want in [src/main.rs](src/main.rs):

```rust
let _ = min_snake::min_snake();
let _ = standard_snake::standard_snake();
```

The board fills your terminal window, so resize it before launching.

## Controls

| Key | Action     |
|-----|------------|
| `w` | Up         |
| `a` | Left       |
| `s` | Down       |
| `d` | Right      |
| `q` | Quit       |

Eat the apples (`O`) to grow. Each apple also speeds the snake up slightly.
You lose by hitting a wall (`+`) or yourself. Your score is printed on exit.

## Dependencies

- [`crossterm`](https://crates.io/crates/crossterm) — terminal control
- [`rand`](https://crates.io/crates/rand) — apple placement
