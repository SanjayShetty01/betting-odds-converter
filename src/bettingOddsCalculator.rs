use std::io;

mod betting_odds_calculator {

    fn fraction_to_decimal(fraction: &str) -> Result<f32, &'static str> {
        let parts: Vec<&str> = fraction.split('/').collect();
        let numerator: f32 = parts[0].parse().unwrap_or_default();
        let denominator: f32 = parts[1].parse().unwrap_or_default();

        if denominator != 0.0 {
            Ok(numerator / denominator)
        } else {
            Err("Division by zero")
        }
    }

    pub fn decimal_prob_calc() {
        println!("\x1b[34mEnter the Decimal Odds:");
        
        let mut user_entered_decimal_value = String::new();
        io::stdin().read_line(&mut user_entered_decimal_value)
            .expect("Failed to read input");
        
        let user_entered_decimal_value: f32 = match user_entered_decimal_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[31mPlease Enter a Numeric Value for Decimal Odds");
                return;
            }
        };

        println!("\x1b[32mCalculating the Odds");
        let ans = prob_functions::decimal_prob(user_entered_decimal_value);
        println!("\x1b[32mThe Implied Probability is {}", ans);
    }

    pub fn money_prob_calc() {
        println!("\x1b[34mEnter the Moneyline:");

        let mut user_entered_moneyline_value = String::new();
        io::stdin().read_line(&mut user_entered_moneyline_value)
            .expect("Failed to read input");

        let user_entered_moneyline_value: f32 = match user_entered_moneyline_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[31mPlease Enter an Integer Value for Moneyline Odds");
                return;
            }
        };

        println!("\x1b[32mCalculating the Odds");
        let ans = prob_functions::moneyline_prob(user_entered_moneyline_value);
        println!("\x1b[32mThe Implied Probability is {}", ans);
    }

    pub fn fraction_prob_calc() {
        println!("\x1b[34mEnter the Fractional Odds:");

        let mut user_entered_fraction_value = String::new();
        io::stdin().read_line(&mut user_entered_fraction_value)
            .expect("Failed to read input");

        let user_entered_fraction_value = user_entered_fraction_value.trim();

        if is_fraction(user_entered_fraction_value) {
            println!("\x1b[32mCalculating the Odds");

            match fraction_to_decimal(user_entered_fraction_value) {
                Ok(value) => {
                    let ans = prob_functions::fraction_prob(value);
                    println!("\x1b[32mThe Implied Probability is {}", ans);
                }
                Err(_) => println!("\x1b[31mError: Invalid fraction or division by zero."),
            }
        } else {
            println!("\x1b[31mEnter Fractional Odds in a Proper Format");
        }
    }

    fn is_fraction(input: &str) -> bool {
        let fraction_pattern = regex::Regex::new(r"^\d+/\d+$").unwrap();
        fraction_pattern.is_match(input)
    }
}

