fn round_to_two_decimal_places(float_number :f32) -> f32{
    (float_number * 100.0).round() / 100.0
}

pub fn moneyline_prob(moneyline :f32) -> f32 {
    let probability = match moneyline {
        moneylineValue if moneylineValue > 0.0 => (100.0 / (moneyline + 100.0)) * 100,
        _ => ((moneyline.abs()) / (moneyline.abs() + 100)) * 100
    };
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