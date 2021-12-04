struct Reinder {
    speed: usize,
    runtime: usize,
    sleeptime: usize,
}

impl Reinder {
    pub fn at(&self, s: usize) -> usize {
        let cycle = self.runtime + self.sleeptime;
        let cycle_count = s / cycle;
        let cycle_ran = cycle_count * self.runtime * self.speed;
        let rest = (s % cycle).min(self.runtime) * self.speed;
        cycle_ran + rest
    }
}

fn main() {
    run(
        r#"
Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
"#,
        1000,
    );
    run(&std::fs::read_to_string("input").unwrap(), 2503)
}

fn run(s: &str, t: usize) {
    let reinders: Vec<_> = s
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut split = line.split(" ");
            let speed = split.nth(3).unwrap().parse::<usize>().unwrap();
            let runtime = split.nth(2).unwrap().parse::<usize>().unwrap();
            let sleeptime = split.nth(6).unwrap().parse::<usize>().unwrap();
            Reinder {
                speed,
                runtime,
                sleeptime,
            }
        }).collect();
    println!("{}", reinders.iter().map(|r| r.at(t)).max().unwrap());
    let mut score = vec![0; reinders.len()];
    for i in 1..t {
        let max = reinders.iter().map(|r| r.at(i)).max().unwrap();
        for (ix, r) in reinders.iter().enumerate() {
            if r.at(i) == max {
                score[ix] += 1;
            }
        }
    }
    println!("{}", score.iter().max().unwrap());
}
