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
        symbols
            .entry(split.nth(1).unwrap().to_string())
            .or_insert(l);
    }
    println!("symbols: {:?}", symbols);
    let mut dist = vec![vec!(0; symbols.len()); symbols.len()];
    for line in ::std::fs::read_to_string("input").unwrap().split("\n") {
        if line.len() == 0 {
            continue;
        }
        let mut split = line.split(" ");
        let a = symbols[split.next().unwrap()];
        let b = symbols[split.nth(1).unwrap()];
        let d = split.nth(1).unwrap().parse::<usize>().unwrap();
        dist[a][b] = d;
        dist[b][a] = d;
    }
    let mut vec:Vec<usize> = (0..symbols.len()).collect();
    let heap = permutohedron::Heap::new(&mut vec);
    let best:usize = heap.map(|data| {
        data.iter().zip(data.iter().skip(1)).map(|(&a,&b)| dist[a][b]).sum()
    }).min().unwrap();
    println!("best: {:?}", best);
    let mut vec:Vec<usize> = (0..symbols.len()).collect();
    let heap = permutohedron::Heap::new(&mut vec);
    let best:usize = heap.map(|data| {
        data.iter().zip(data.iter().skip(1)).map(|(&a,&b)| dist[a][b]).sum()
    }).max().unwrap();
    println!("worst: {:?}", best);
}
