use std::io::{self, Write};

mod min_snake;
mod standard_snake;


fn main() {
    let matches = clap::Command::new("mini-snake")
        .about("A small terminal Snake game.")
        .arg(
            clap::Arg::new("impl")
                .value_parser(["std", "min"])
                .help("Which implementation to run: 'std' or 'min'"),
        )
        .get_matches();

    let choice = match matches.get_one::<String>("impl") {
        Some(s) => s.clone(),
        None => prompt_choice(),
    };

    match choice.as_str() {
        "std" => {
            let _ = standard_snake::standard_snake();
        }
        "min" => {
            let _ = min_snake::min_snake();
        }
        _ => unreachable!(),
    }
}

fn prompt_choice() -> String {
    loop {
        println!("What option do you want to run?");
        println!("0 - Minimal");
        println!("1 - Standard");
        print!("> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            continue;
        }

        match line.trim() {
            "0" => return "min".to_string(),
            "1" => return "std".to_string(),
            _ => println!("Invalid selection, try again.\n"),
        }
    }
}
