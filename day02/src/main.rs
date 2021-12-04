fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let area: usize = s
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| -> usize {
            let values: Vec<usize> = line
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let main = 2 * (values[0] * values[1] + values[0] * values[2] + values[1] * values[2]);
            let slack: usize =
                values.iter().cloned().product::<usize>() / values.iter().cloned().max().unwrap();
            main + slack
        }).sum();
    println!("1: {}", area);
    let length: usize = s
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| -> usize {
            let values: Vec<usize> = line
                .split("x")
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            let main: usize =
                2 * (values.iter().cloned().sum::<usize>() - values.iter().cloned().max().unwrap());
            let bow: usize = values.iter().cloned().product::<usize>();
            main + bow
        }).sum();
    println!("2: {}", length);
}
