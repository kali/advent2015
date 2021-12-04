fn main() {
    let mut state = vec![vec!(false; 1000); 1000];
    let s = std::fs::read_to_string("input").unwrap();
    for line in s.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let action: Box<fn(bool) -> bool> = if line.starts_with("toggle") {
            Box::new(|x| !x)
        } else if line.starts_with("turn on") {
            Box::new(|_| true)
        } else {
            Box::new(|_| false)
        };
        let mut coords = line
            .split(" ")
            .filter(|c| c.chars().next().unwrap().is_numeric())
            .map(|s| {
                let mut coords = s.split(",");
                (
                    coords.next().unwrap().parse::<usize>().unwrap(),
                    coords.next().unwrap().parse::<usize>().unwrap(),
                )
            });
        let start = coords.next().unwrap();
        let end = coords.next().unwrap();
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                state[y][x] = action(state[y][x]);
            }
        }
    }
    println!("1: {}", state.iter().map(|l| l.iter().filter(|v|**v).count()).sum::<usize>());
    let mut state = vec![vec!(0; 1000); 1000];
    let s = std::fs::read_to_string("input").unwrap();
    for line in s.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let action: Box<fn(usize) -> usize> = if line.starts_with("toggle") {
            Box::new(|x| x+2)
        } else if line.starts_with("turn on") {
            Box::new(|x| x+1)
        } else {
            Box::new(|x| x.saturating_sub(1))
        };
        let mut coords = line
            .split(" ")
            .filter(|c| c.chars().next().unwrap().is_numeric())
            .map(|s| {
                let mut coords = s.split(",");
                (
                    coords.next().unwrap().parse::<usize>().unwrap(),
                    coords.next().unwrap().parse::<usize>().unwrap(),
                )
            });
        let start = coords.next().unwrap();
        let end = coords.next().unwrap();
        for x in start.0..=end.0 {
            for y in start.1..=end.1 {
                state[y][x] = action(state[y][x]);
            }
        }
    }
    println!("2: {}", state.iter().map(|l| l.iter().sum::<usize>()).sum::<usize>());
}
