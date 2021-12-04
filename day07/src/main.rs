use std::collections::HashMap;

fn resolve(over: &HashMap<String, u16>, s: &HashMap<String, u16>, v: &str) -> Option<u16> {
    if v.chars().next().unwrap().is_numeric() {
        Some(v.parse().unwrap())
    } else {
        over.get(v).or(s.get(v)).cloned()
    }
}

fn run(over:&HashMap<String, u16>) -> u16 {
    let ref mut s = HashMap::new();
    loop {
        for line in std::fs::read_to_string("input")
            .unwrap()
            .split("\n")
            .filter(|line| line.len() > 0)
        {
            let mut split = line.split(" -> ");
            let value = split.next().unwrap();
            let target = split.next().unwrap();
            let split: Vec<&str> = value.split(" ").collect();
            let value = match &*split {
                [v] => resolve(over, s, v),
                ["NOT", v] => resolve(over, s, v).map(|x| !x),
                [a, op, b] => {
                    if let (Some(a), Some(b)) = (resolve(over, s, a), resolve(over, s, b)) {
                        Some(match *op {
                            "AND" => a & b,
                            "OR" => a | b,
                            "LSHIFT" => a << b,
                            "RSHIFT" => a >> b,
                            _ => panic!(),
                        })
                    } else {
                        None
                    }
                }
                _ => panic!(),
            };
            if let Some(value) = value {
                s.insert(target.to_string(), value);
            }
        }
        if let Some(a) = s.get("a") {
            return *a;
        }
    }
}

fn main() {
    let mut over = HashMap::new();
    let a = run(&over);
    println!("a: {}", a);
    over.insert("b".into(), a);
    let a = run(&over);
    println!("a: {}", a);
}
