use colored::*;
mod betting_odds_calculator;
mod prob_functions;
mod utils;
use figlet_rs;
use std::io;

fn main() {
    let standard_font = figlet_rs::FIGfont::standard()
                                                        .unwrap();
    let welcome_note = standard_font
                                    .convert("Betting Odds Converter");

    assert!(welcome_note.is_some());
    print!("{}", welcome_note.unwrap());
    println!("{}", "Calculates Implied Probability".blue());

    let mut exit: bool = false;

    while  !exit {
 
        let mut num = String::new();

        utils::display_main_menu();

        io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the number");

        utils::which_calc_decider(&num.trim().parse::<i32>());

        println!("Press 'x' to exit or any other key to continue: ");

        let mut final_call = String::new();
            io::stdin()
                .read_line(&mut final_call)
                .expect("Failed to read input");

        if final_call.trim().eq_ignore_ascii_case("x") {
            exit = true; 
    }
}
}