extern crate permutohedron;

use std::collections::HashMap;

fn main() {
    let mut symbols = HashMap::<String, usize>::new();
    for line in ::std::fs::read_to_string("input").unwrap().split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut split = line.split(" ");
        let l = symbols.len();
        symbols
            .entry(split.next().unwrap().to_string())
            .or_insert(l);
        let l = symbols.len();
        let last = split.last().unwrap();
        let last = last[..(last.len()-1)].to_string();
        symbols.entry(last).or_insert(l);
    }
    let mut dist = vec![vec!(0; symbols.len()); symbols.len()];
    for line in ::std::fs::read_to_string("input").unwrap().split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut split = line.split(" ");
        let a = symbols[split.next().unwrap()];
        let verb = split.nth(1).unwrap();
        let weight = split.next().unwrap().parse::<isize>().unwrap();
        let b = split.nth(6).unwrap();
        let b = &b[0..b.len()-1];
        let b = symbols[b];
        let score = weight * if verb == "gain" { 1isize } else { -1 };
        dist[a][b] += score;
        dist[b][a] += score;
    }
    println!("dist: {:?}", dist);
    let mut vec:Vec<usize> = (0..symbols.len()).collect();
    let heap = permutohedron::Heap::new(&mut vec);
    let best:isize = heap.map(|data| {
        data.iter().zip(data.iter().skip(1)).map(|(&a,&b)| dist[a][b]).sum::<isize>() + dist[data[0]][data[data.len()-1]]
    }).max().unwrap();
    println!("best: {:?}", best);
    let mut vec:Vec<usize> = (0..symbols.len()).collect();
    let heap = permutohedron::Heap::new(&mut vec);
    let best:isize = heap.map(|data| {
        data.iter().zip(data.iter().skip(1)).map(|(&a,&b)| dist[a][b]).sum::<isize>()
    }).max().unwrap();
    println!("best: {:?}", best);
}
