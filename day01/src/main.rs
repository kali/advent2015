fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let a1 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }).sum::<isize>();
    println!("1: {}", a1);
    let a2 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }).scan(0, |acc,it| { *acc += it; Some(*acc) })
        .enumerate()
        .find(|(_,level)| *level == -1)
        .unwrap().0+1;
    println!("2: {}", a2);
}
