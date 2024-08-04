use std::fmt::format;

use colored::*;
mod betting_odds_calculator;
mod prob_functions;
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
        let calculator_type: [&str; 3] = [
            "Moneyline to Implied probability",
            "Decimal Odds to Implied probability",
            "Fractional Odds to Implied probability"];

        println!("{}", "Select the Converter".blue());

        for (index, &item) in calculator_type.iter().enumerate() {
            let numbered = format!("{}.", index + 1);
            println!("{}, {}", numbered, item);
        }

    }
}
