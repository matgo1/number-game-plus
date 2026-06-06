use rand::RngExt;

pub fn generate_random_number(input_range: &std::ops::RangeInclusive<u16>) -> u16 {
    let mut rng = rand::rng();
    rng.random_range(*input_range.start()..=*input_range.end())
}
