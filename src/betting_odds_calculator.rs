use std::io;
use colored::* ;
use crate::prob_functions;

pub fn decimal_prob_calc() {
    println!("{}", "Enter the Decimal Odds: ".blue());
    
    let mut user_entered_decimal_value = String::new();

    io::stdin()
        .read_line(&mut user_entered_decimal_value)
        .expect("Failed to read Decimal Odds");

    match user_entered_decimal_value.trim().parse::<f32>() {
        Ok(value) => {
            println!("{}", "Calculating the Odds...".green());
            let prob : f32 = prob_functions::decimal_prob(value);
            println!("{}", format!("The Implied Probability is {}", prob).green());
        }

        Err(_) => {
            println!("{}", "Please Enter a Numeric Value for Decimal Odds".red())
        }
    }
}

pub fn money_prob_calc() {
    println!("{}", "Enter the Moneyline: ".blue());

    let mut user_entered_moneyline_value: String = String::new();

    io::stdin()
        .read_line(&mut user_entered_moneyline_value)
        .expect("Failed to read Decimal Odds");

    match user_entered_moneyline_value.trim().parse::<f32>() {
        Ok(value) => {
            println!("{}", "Calculating the Odds...".green());
            let prob : f32 = prob_functions::moneyline_prob(value);
            println!("{}", format!("The Implied Probability is {}", prob).green());
        }

        Err(_) => {
            println!("{}", "Please Enter an Integer Value for Moneyline Odds".red())
        }
    }
}

// Figure out fraction in Rust
pub fn fraction_prob_calc() {
    unimplemented!()
}