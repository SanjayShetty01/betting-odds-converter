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

    let mut wager = utils::get_wager();

    let mut exit: bool = false;

    while  !exit {
        
        let mut num = String::new();

        utils::display_main_menu();

        io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the number");

        let parsed_num = num.trim().parse::<i32>();
        
        match parsed_num {
            Ok(value) => {
                utils::which_calc_decider(value, wager);
                },
                Err(_) => {
                    println!("Invalid number! Please try again.");  
                }
            
            };

        println!("Press 'x' to exit, 'c' to change the wager or any other key to continue: ");

        let mut final_call = String::new();
            io::stdin()
                .read_line(&mut final_call)
                .expect("Failed to read input");

        if final_call.trim().eq_ignore_ascii_case("x") {
            exit = true; 
        } else if (final_call.trim().eq_ignore_ascii_case("c")) {
            wager = utils::get_wager();
        }
}
}