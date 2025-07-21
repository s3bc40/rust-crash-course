pub fn first(t: (bool, u32, char)) -> bool {
    t.0
}

pub fn last(t: (bool, u32, char)) -> char {
    let (_, _, last) = t;
    last
}

pub fn swap(t: (u32, u32)) -> (u32, u32) {
    let (first, second) = t;
    (second, first)
}
