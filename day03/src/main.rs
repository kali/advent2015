use std::collections::HashSet;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut hash = HashSet::new();
    let mut position = (0isize, 0isize);
    for dir in s.chars() {
        match dir {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 -= 1,
            'v' => position.1 += 1,
            _ => {}
        }
        hash.insert(position);
    }
    println!("1: {}", hash.len());

    let mut hash = HashSet::new();
    let mut santa = (0isize, 0isize);
    let mut bot = (0isize, 0isize);
    for (ix, dir) in s.chars().enumerate() {
        let position: &mut (isize, isize) = if ix % 2 == 0 { &mut santa } else { &mut bot };
        match dir {
            '<' => position.0 -= 1,
            '>' => position.0 += 1,
            '^' => position.1 -= 1,
            'v' => position.1 += 1,
            _ => {}
        }
        hash.insert(*position);
    }
    println!("1: {}", hash.len());
}
