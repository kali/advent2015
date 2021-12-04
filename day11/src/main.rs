extern crate itertools;
use itertools::Itertools;

fn main() {
    let mut input = b"hxbxwxba".to_vec();
    loop {
        let mut p = input.len() - 1;
        loop {
            if input[p] == b'z' {
                input[p] = b'a';
                p -= 1;
            } else {
                input[p] += 1;
                break;
            }
        }
        let three = input
            .iter()
            .zip(input.iter().skip(1))
            .zip(input.iter().skip(2))
            .any(|((&a, &b), &c)| a == b - 1 && a == c - 2);
        let iol = !input.iter().any(|c| b"iol".contains(c));
        let pairs = input
            .iter()
            .zip(input.iter().skip(1))
            .enumerate()
            .filter(|(ix, (a, b))| a == b)
            .map(|(ix, _)| ix)
            .tuple_combinations()
            .any(|(ix1, ix2)| (ix1.max(ix2))-(ix1.min(ix2)) > 1);
        if three && iol && pairs {
            println!("{}", std::str::from_utf8(&*input).unwrap());
        }
    }
}
