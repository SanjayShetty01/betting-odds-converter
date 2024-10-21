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
            println!("{}", format!("The Implied Probability is {}%", prob).green());
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
            println!("{}", format!("The Implied Probability is {}%", prob).green());
        }

        Err(_) => {
            println!("{}", "Please Enter an Integer Value for Moneyline Odds".red())
        }
    }
}

pub fn fraction_prob_calc() {
    println!("{}", "Enter the Fractional Odds (e.g., 3/4): ".blue());
    
    let mut user_entered_fraction = String::new();

    io::stdin()
    .read_line(&mut user_entered_fraction)
    .expect("Failed to read Fractional Odds");

    let input = user_entered_fraction.trim();

    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() == 2 {
        match (parts[0].trim().parse::<f32>(), parts[1].trim().parse::<f32>()) {
            (Ok(numerator), Ok(denominator)) if denominator != 0.0 => {
                let value = numerator / denominator;
                let prob: f32 = prob_functions::fraction_prob(value);
                println!("{}", format!("The Implied Probability is {}%", prob).green());
            }
            _ => {
                println!("{}", "Invalid fraction entered. Please try again.".red());
            }
        }
    } else {
        println!("{}", "Invalid fraction format. Please enter in the form 'numerator/denominator'.".red());
    }

}