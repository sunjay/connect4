extern crate ansi_term;

mod connect4;

use std::iter;

use ansi_term::Colour::{Red, Yellow, White};

use connect4::Connect4;
use connect4::Piece::*;

fn main() {
    let game = Connect4::new();

    render(game);
}

fn render(game: Connect4) {
    println!("{}  \n", ["A", "B", "C", "D", "E", "F", "G"].into_iter().fold(String::new(), |acc, s| {
        format!("{}   {}", acc, s)
    }));

    for row in game.to_rows() {
        println!("{} \u{2502}", row.iter().map(|tile| {
            tile.map_or_else(|| {
                White.bold().paint("\u{25CB}")
            }, |piece| match piece {
                PieceX => Red.bold().paint("\u{25CF}"),
                PieceO => Yellow.bold().paint("\u{25CF}"),
            })
        }).fold(String::new(), |acc, s| {
            format!("{} \u{2502} {}", acc, s)
        }));
    }

    render_line();
}

fn render_line() {
    println!(" \u{2534}{}", iter::repeat("\u{2500}\u{2501}\u{2500}\u{2534}").take(7).fold(String::new(), |acc, s| {
        acc + s
    }));
}
