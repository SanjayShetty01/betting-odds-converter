fn round_to_two_decimal_places(float_number :f32) -> f32{
    (float_number * 100.0).round() / 100.0
}

pub fn moneyline_prob(moneyline :f32) -> f32 {
    let probability: f32= match moneyline {
        moneyline_value if moneyline_value > 0.0 => (100.0 / (moneyline + 100.0)) * 100.0,
        _ => ((moneyline.abs()) / (moneyline.abs() + 100.0)) * 100.0
    };

    return probability;
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