use crate::betting_odds_calculator;
use colored::*;
use std::io;
use tabled::{  settings::{style::Style, themes::Theme, Color}, 
    Tabled, Table};

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

    if wager.trim().is_empty() {
        println!("No input provided, using default wager of 100");
        return 100.0;
    }

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
    value: String,
}

pub fn display_metrics_table(implied_prob: f64, payout: f64, return_percentage: f64) {
    let metrics = vec![
        MetricRow { metric: "Implied Probability (%)", value: format!("{:.2}",implied_prob) },
        MetricRow { metric: "Payout", value: format!("{:.2}", payout) },
        MetricRow { metric: "Return (%)", value: format!("{:.2}", return_percentage) },
    ];

    let mut style = Theme::from(Style::extended());
    style.set_colors_top(Color::FG_RED);
    style.set_colors_bottom(Color::FG_CYAN);
    style.set_colors_left(Color::FG_BLUE);
    style.set_colors_right(Color::FG_GREEN);
    style.set_colors_corner_top_left(Color::FG_BLUE);
    style.set_colors_corner_top_right(Color::FG_RED);
    style.set_colors_corner_bottom_left(Color::FG_CYAN);
    style.set_colors_corner_bottom_right(Color::FG_GREEN);
    style.set_colors_intersection_bottom(Color::FG_CYAN);
    style.set_colors_intersection_top(Color::FG_RED);
    style.set_colors_intersection_right(Color::FG_GREEN);
    style.set_colors_intersection_left(Color::FG_BLUE);
    style.set_colors_intersection(Color::FG_MAGENTA);
    style.set_colors_horizontal(Color::FG_MAGENTA);
    style.set_colors_vertical(Color::FG_MAGENTA);

    let table = Table::new(metrics).with(style).to_string();

    println!("{table}")
}