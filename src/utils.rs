use crate::betting_odds_calculator;
use colored::*;
use std::io;
use tabled::{Tabled, Table};

pub fn which_calc_decider(choice: i32, wager: f64) {
    match choice {
        1 => betting_odds_calculator::money_prob_calc(wager),
        2 => betting_odds_calculator::decimal_prob_calc(wager),
        3 => betting_odds_calculator::fraction_prob_calc(wager),
        _ => println!("Please choose a valid option between 1-3."),
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

pub fn get_wager() -> f64 {
    println!("Enter your wager (Default is 100): ");

    let mut wager = String::new();
    io::stdin().read_line(&mut wager).expect("Failed to read the number");

    match wager.trim().parse::<f64>() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input, using default wager of 100");
            100.0
        }
    }
}

#[derive(Tabled)]
struct MetricRow {
    metric: &'static str,
    value: f32,
}

pub fn display_metrics_table(implied_prob: f32, payout: f32, return_percentage: f32) -> Table {
    let metrics = vec![
        MetricRow { metric: "Implied Probability (%)", value: implied_prob },
        MetricRow { metric: "Payout", value: payout },
        MetricRow { metric: "Return (%)", value: return_percentage },
    ];

    let table = Table::new(metrics);

    return table
}