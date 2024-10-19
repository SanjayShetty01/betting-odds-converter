use crate::betting_odds_calculator;
use std::num::ParseIntError;
use colored::*;

pub fn which_calc_decider(choice: &Result<i32, ParseIntError>) {
    match choice {
        Ok(value) => match value {
            1 => println!("Moneyline to Implied Probability"),
            2 => println!("Decimal Odds to Implied Probability"),
            3 => println!("Fractional Odds to Implied Probability"),
            _ => println!("Please choose a valid option between 1-3."),
        },
        Err(_) => println!("Invalid number! Please try again."),
    }
}

pub fn display_main_menu() {
    let odds = vec![
        "Moneyline to Implied probability",
        "Decimal Odds to Implied probability",
        "Fractional Odds to Implied probability",
    ];

    println!("{}", "Select the Converter".blue());
    for (index, odd) in odds.iter().enumerate() {
        println!("{}. {}", index + 1, odd);
    }
}