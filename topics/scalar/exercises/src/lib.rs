pub fn eq(c1: char, c2: char) -> bool {
    println!("char1: {} char2: {}", c1, c2);
    c1 == c2
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    println!("{0} {1} {2}", x, y, z);
    x + y + z
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    x as f32 + y as f32 + z
}
