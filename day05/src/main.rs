fn nice_1(s: &str) -> bool {
    let voyels = s
        .chars()
        .filter(|v| "aeiou".contains(*v))
        .count();
    let stut = s.chars().skip(1).zip(s.chars()).any(|(a,b)| a == b);
    let pairs = ["ab", "cd", "pq", "xy"].iter().any(|pair| s.contains(pair));
    voyels >= 3 && stut && !pairs
}

fn nice_2(s: &str) -> bool {
    let s = s.as_bytes();
    let len = s.len();
    if len < 2 {
        return false;
    }
    let pairs = (0..(len-2)).any(|ix| {
        ((ix+2)..(len-1)).any(|i| s[i] == s[ix] && s[i+1] == s[ix+1])
    });
    let aba = s.iter().skip(2).zip(s.iter()).any(|(a,b)| a == b);
    pairs && aba
}

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("1: {}", s.split("\n").filter(|l| nice_1(l)).count());
    println!("2: {}", s.split("\n").filter(|l| nice_2(l)).count());
}
