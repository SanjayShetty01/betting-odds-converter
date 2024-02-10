mod prob_functions {
    fn round_to_second_decimal(float_num: f32) -> f32 {
        (float_num * 100.0).round() / 100.0
    }

    pub fn moneyline_prob(moneyline: f32) -> f32 {
        match moneyline {
            moneyline_value if moneyline_value > 0.0 => round_to_second_decimal((100.0 / (moneyline + 100.0)) * 100.0),
            _ => round_to_second_decimal(((moneyline * -1.0) / ((moneyline * -1.0) + 100.0)) * 100.0),
        }
    }

    pub fn decimal_prob(decimal: f32) -> f32 {
        round_to_second_decimal((1.0 / decimal) * 100.0)
    }

    pub fn fraction_prob(fractions: f32) -> f32 {
        round_to_second_decimal((1.0 / fractions) * 100.0)
    }
}