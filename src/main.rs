extern crate ansi_term;

mod connect4;

use std::iter;

use ansi_term::Colour::{Red, Yellow, White};

use connect4::{Connect4, Piece, Error};
use connect4::Piece::*;

fn main() {
    let mut game = Connect4::new();
    game.drop_piece(0).unwrap();
    game.drop_piece(0).unwrap();
    game.drop_piece(1).unwrap();
    game.drop_piece(2).unwrap();
    game.drop_piece(2).unwrap();

    render(game);
}

fn render(game: Connect4) {
    println!("{}  \n", ["A", "B", "C", "D", "E", "F", "G"].into_iter().fold(String::new(), |acc, s| {
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
