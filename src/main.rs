extern crate ansi_term;

mod connect4;

use std::iter;
use std::io::{self, Write};
use std::process::Command;

use ansi_term::Colour::{Red, Yellow, White};

use connect4::{Connect4, Piece, Error};
use connect4::Piece::*;

fn main() {
    let mut game = Connect4::new();
    let mut error = None;

    let stdin = io::stdin();

    loop {
        println!();
        Command::new("clear").status().unwrap();
        render(&game);

        println!();
        render_error(error);
        println!(" Current piece: {}", format_piece(game.current_piece()));
        print!(" Drop piece into column (A-G): ");
        flush_stdout();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Could not read input");

        let col = match &buffer.trim().to_lowercase()[..] {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            _ => {
                error = Some(Error::InvalidMove);
                continue;
            }
        };

        error = None;
        if let Err(err) = game.drop_piece(col) {
            error = Some(err);
        }
    }
}

fn render_error(error: Option<Error>) {
    println!("{}", Red.bold().paint(match error {
        Some(Error::InvalidMove) => " Invalid move",
        Some(Error::NoSpaceLeftInColumn) => " No space available in this column",
        None => "",
    }));
}

fn render(game: &Connect4) {
    println!("\n{}  \n", ["A", "B", "C", "D", "E", "F", "G"].into_iter().fold(String::new(), |acc, s| {
        format!("{}   {}", acc, s)
    }));

    println!("{}  ", iter::repeat("\u{1F823}").take(7).into_iter().fold(String::new(), |acc, s| {
        format!("{}   {}", acc, s)
    }));

    for row in game.to_rows() {
        render_line();

        println!("{} \u{2502}", row.iter().map(|tile| {
            tile.map_or_else(format_empty, format_piece)
        }).fold(String::new(), |acc, s| {
            format!("{} \u{2502} {}", acc, s)
        }));
    }

    render_bottom_line();
}

fn format_empty() -> String {
    format!("{}", White.bold().paint("\u{25CB}"))
}

fn format_piece(piece: Piece) -> String {
    format!("{}", match piece {
        PieceX => Red.bold().paint("\u{25CF}"),
        PieceO => Yellow.bold().paint("\u{25CF}"),
    })
}

fn render_line() {
    println!(" \u{253C}{}", iter::repeat("\u{2500}\u{2500}\u{2500}\u{253C}").take(7).fold(String::new(), |acc, s| {
        acc + s
    }));
}

fn render_bottom_line() {
    println!(" \u{2534}{}", iter::repeat("\u{2500}\u{2501}\u{2500}\u{2534}").take(7).fold(String::new(), |acc, s| {
        acc + s
    }));
}

fn flush_stdout() {
    io::stdout().flush().ok().expect("Could not flush stdout");
}
