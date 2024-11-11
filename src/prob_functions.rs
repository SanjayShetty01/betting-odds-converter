fn round_to_two_decimal_places(float_number :f32) -> f32{
    (float_number * 100.0).round() / 100.0
}

pub fn moneyline_prob(moneyline :f32) -> f32 {
    let probability: f32= match moneyline {
        moneyline_value if moneyline_value > 0.0 => (100.0 / (moneyline + 100.0)) * 100.0,
        _ => ((moneyline.abs()) / (moneyline.abs() + 100.0)) * 100.0
    };

    return round_to_two_decimal_places(probability);
}

fn calculate_prob(odds: f32) -> f32 {
    round_to_two_decimal_places((1.0 / odds) * 100.0)
}

pub fn decimal_prob(decimal: f32) -> f32 {
    calculate_prob(decimal)
}

pub fn fraction_prob(fractions: f32) -> f32 {
    calculate_prob(fractions)
}

pub fn calculate_payout(implied_prob : f32, wager: f64) -> f64 {
 let odds : f64 =  1.0 / (implied_prob as f64);

 odds * wager
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_to_two_decimal_places() {
        assert_eq!(round_to_two_decimal_places(1.23456), 1.23);
        assert_eq!(round_to_two_decimal_places(1.235), 1.24);
        assert_eq!(round_to_two_decimal_places(100.0), 100.0);
    }

    #[test]
    fn test_moneyline_prob_positive() {
        let moneyline: f32 = 150.0;
        let expected_prob: f32 = (100.0 / (moneyline + 100.0)) * 100.0;
        assert_eq!(moneyline_prob(moneyline), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_moneyline_prob_negative() {
        let moneyline: f32 = -150.0;
        let expected_prob: f32 = (moneyline.abs() / (moneyline.abs() + 100.0)) * 100.0;
        assert_eq!(moneyline_prob(moneyline), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_decimal_prob() {
        let decimal: f32 = 2.5;
        let expected_prob: f32 = (1.0 / decimal) * 100.0;
        assert_eq!(decimal_prob(decimal), round_to_two_decimal_places(expected_prob));
    }

    #[test]
    fn test_fraction_prob() {
        let fractions: f32 = 5.0 / 2.0;
        let expected_prob: f32 = (1.0 / fractions) * 100.0;
        assert_eq!(fraction_prob(fractions), round_to_two_decimal_places(expected_prob));
    }
}