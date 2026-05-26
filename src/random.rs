use rand::RngExt;

pub fn generate_rundom_number(start: u32, end: u32) -> u32 {
    let mut rng = rand::rng();
    rng.random_range(start..=end)
}
