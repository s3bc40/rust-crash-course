pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let parsed_a: u32 = a.parse().expect("Failed to parse variable");
    let parse_b: u32 = b.parse().expect("Failed to parse variable");
    parsed_a + parse_b
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
}
