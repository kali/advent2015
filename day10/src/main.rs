extern crate itertools;
use itertools::Itertools;

use std::fmt::Write;

fn main() {
    let mut it = "1321131112".to_string();

    for _ in 0..50 {
        let mut next = String::new();
        {
            let groups = it.chars().group_by(|&c| c);
            for (key, group) in groups.into_iter() {
                write!(&mut next, "{}", group.count());
                write!(&mut next, "{}", key);
            }
        }
        it = next;
    }

    println!("{}", it.len());
}
