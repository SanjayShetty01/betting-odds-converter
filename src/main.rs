use std::fmt::format;
use colored::*;
mod betting_odds_calculator;
mod prob_functions;
mod utils;
use figlet_rs;

fn main() {
    let standard_font = figlet_rs::FIGfont::standard()
                                                        .unwrap();
    let welcome_note = standard_font
                                    .convert("Betting Odds Converter");

    assert!(welcome_note.is_some());
    print!("{}", welcome_note.unwrap());
    println!("{}", "Calculates Implied Probability".blue());

    let exit: bool = false;

    while  !exit {
        utils::display_main_menu();


    }
}
