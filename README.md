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

Pick which version to run with an argument:

```sh
cargo run -- std   # standard (readable) implementation
cargo run -- min   # minimal implementation
```

If you run without an argument, you'll be prompted to choose:

```sh
cargo run
# What option do you want to run?
# 0 - Minimal
# 1 - Standard
# >
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
