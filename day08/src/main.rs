fn s1(s: &str) -> usize {
    s.split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let used = line.len();
            let mut inner = line[1..used - 1].bytes();
            let mut wasted = 2;
            while let Some(c) = inner.next() {
                if c == b'\\' {
                    if inner.next().unwrap() == b'x' {
                        inner.next();
                        inner.next();
                        wasted += 3;
                    } else {
                        wasted += 1;
                    }
                }
            }
            wasted
        }).sum::<usize>()
}

fn s2(s: &str) -> usize {
    s.split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let len = line.len();
            let mut used = 2;
            let mut line = line.chars();
            while let Some(c) = line.next() {
                if c == '\"' {
                    used += 2;
                } else if c == '\\' {
                    used += 2;
                } else {
                    used += 1;
                }
            }
            used - len
        }).sum::<usize>()
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("1: {}", s1(&s));
    println!("2: {}", s2(&s));
}

#[test]
fn t2() {
    assert_eq!(s2("\"\""), 4);
    assert_eq!(s2("\"abc\""), 4);
    assert_eq!(s2("\"aaa\\\"aaa\""), 6);
    assert_eq!(s2("\"\\x27\""), 5);
}
