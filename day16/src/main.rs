fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(&s.trim()));
}

fn run(s: &str) -> usize {
    'sue: for sue in s.split("\n") {
        let mut s = sue.split(" ");
        let id = s.nth(1).unwrap();
        println!("Sue {}", id);
        for _ in 0..3 {
            let clue = s.next().unwrap().trim_matches(':');
            let count = s
                .next()
                .unwrap()
                .trim_matches(',')
                .parse::<usize>()
                .unwrap();
            println!("clue {:?} {}", clue, count);
            let ok = match clue {
                "children" => count == 3,
                "cats" => count >= 7,
                "samoyeds" => count == 2,
                "pomeranians" => count <= 3,
                "akitas" => count == 0,
                "vizslas" => count == 0,
                "goldfish" => count <= 5,
                "trees" => count >= 3,
                "cars" => count == 2,
                "perfumes" => count == 1,
                _ => panic!()
            };
            if !ok {
                continue 'sue;
            }
        }
        return id.trim_matches(':').parse::<usize>().unwrap();
    }
    panic!()
}
