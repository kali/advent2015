extern crate aho_corasick;
extern crate pathfinding;

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aho_corasick::Automaton;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    let mut subs = vec![];
    let mut lines = s.trim().split("\n");
    let mut molecule = String::new();
    while let Some(line) = lines.next() {
        if line.contains("=>") {
            let mut toks = line.split(" ");
            subs.push((
                toks.next().unwrap().as_bytes(),
                toks.nth(1).unwrap().as_bytes(),
            ));
        } else if line.len() > 0 {
            molecule = line.to_string();
        }
    }
    println!("'{}'", molecule);
    println!("{:?}", successors(molecule.as_bytes(), &subs).len());
    println!("{:?}", part_2(molecule.as_bytes(), &subs));
}

fn successors(molecule: &[u8], subs: &[(&[u8], &[u8])]) -> Vec<Vec<u8>> {
    let mut results = HashSet::new();
    for sub in subs {
        if sub.0.len() > molecule.len() {
            continue;
        }
        for i in 0..=(molecule.len() - sub.0.len()) {
            if &molecule[i..][..sub.0.len()] == sub.0 {
                let mut gen = (&molecule[..i]).to_vec();
                gen.extend(sub.1);
                gen.extend(&molecule[(i + sub.0.len())..]);
                results.insert(gen);
            }
        }
    }
    results.into_iter().collect()
}

fn part_2_td(molecule: &[u8], subs: &[(&[u8], &[u8])]) -> usize {
    let max_diff = subs.iter().map(|(s, l)| l.len() - s.len()).max().unwrap() as isize;
    let mut generated = HashMap::<Vec<u8>, usize>::new();
    let mut queue: BinaryHeap<(isize, Vec<u8>)> = BinaryHeap::new();
    let aho = aho_corasick::AcAutomaton::new(subs.iter().map(|s| s.1));
    let aho = aho_corasick::FullAcAutomaton::new(aho);
    generated.insert(molecule.to_vec(), 0);
    queue.push((-(molecule.len() as isize), molecule.to_vec()));
    let e = b"e".to_vec();
    println!("max_diff: {}", max_diff);
    let mut i = 0;
    while let Some((s, it)) = queue.pop() {
        if i % 1000 == 0 {
            println!("i: {} s: {} q: {}", i, s, queue.len());
        }
        i += 1;
        let dist = generated[&it];
        for m in aho.find_overlapping(&it) {
            let mut gen = (&it[..m.start]).to_vec();
            gen.extend(subs[m.pati].0);
            gen.extend(&it[m.end..]);
            if gen.iter().filter(|&b| *b == b'e').count() >= 2 {
                continue;
            }
            if !generated.contains_key(&gen) {
                if gen == e {
                    return dist + 1;
                }
                generated.insert(gen.clone(), dist + 1);
                let score = gen.len() as isize + dist as isize * max_diff + 1;
                //                println!(" > {} ({})", std::str::from_utf8(&*gen).unwrap(), score);
                queue.push((-score, gen));
            }
        }
    }
    panic!();
}

fn fixed_letters(subs: &[(&[u8], &[u8])]) -> HashSet<u8> {
    let right_letters:HashSet<u8> = subs.iter().flat_map(|s| s.1.iter().cloned()).collect();
    let left_letters:HashSet<u8> = subs.iter().flat_map(|s| s.0.iter().cloned()).collect();
    right_letters.difference(&left_letters).cloned().collect()
}

fn skeleton(molecule: &[u8], fixed: &HashSet<u8>) -> Vec<u8> {
    molecule.iter().cloned().filter(|c| fixed.contains(c)).collect()
}

fn subseq(haystack: &[u8], needle: &[u8]) -> bool {
    let mut pos = 0;
    for &n in needle {
        if pos == haystack.len() {
            return false;
        }
        while haystack[pos] != n {
            pos += 1;
            if pos == haystack.len() {
                return false;
            }
        }
        pos += 1;
    }
    return true;
}

fn part_2_slow(molecule: &[u8], subs: &[(&[u8], &[u8])]) -> usize {
    let aho = aho_corasick::AcAutomaton::new(subs.iter().map(|s| s.0));
    let aho = aho_corasick::FullAcAutomaton::new(aho);
    let max_diff = subs.iter().map(|(s, l)| l.len() - s.len()).max().unwrap();
    let mut reached = 0;
    let started = std::time::Instant::now();
    let fixed = fixed_letters(&subs);
    println!("fixed: {:?}", fixed);
    let molecule_skeleton = skeleton(molecule, &fixed);
    println!("skel: {:?}", molecule_skeleton);
    pathfinding::directed::astar::astar(
        &b"e".to_vec(),
        |it| aho.find_overlapping(&it).map(|m| {
            let mut gen = (&it[..m.start]).to_vec();
            gen.extend(subs[m.pati].1);
            gen.extend(&it[m.end..]);
            if reached < gen.len() {
                let elapsed = std::time::Instant::elapsed(&started);
                println!("reached {} in {}s", gen.len(), elapsed.as_secs());
                reached = gen.len();
            }
            (gen, max_diff)
        })
        .filter(|it| subseq(&*molecule_skeleton,&skeleton(&*it.0, &fixed)))
        .collect::<Vec<_>>(),
        |it| (molecule.len().saturating_sub(it.len())),
        |it| &**it == molecule
    ).unwrap().0.len() - 1
}

fn part_2(molecule: &[u8], subs: &[(&[u8], &[u8])]) -> usize {
    let n = molecule.iter().filter(|&n| *n == b'n').count();
    let y = molecule.iter().filter(|&n| *n == b'Y').count();
    let len = molecule.iter().filter(|n| **n >= b'A' && **n <= b'Z').count();
    // the "n" rules calls also generated 2n caps (Rn and Ar), plus all the y,
    // and as many regular symbols as n+y from n regular symbols 
    println!("n: {} y: {} len: {}", n, y, len);
    len - 1 - 2*n - 2*y
}

#[test]
fn test() {
    let subs: &[(&[u8], &[u8])] = &[(b"H", b"HO"), (b"H", b"OH"), (b"O", b"HH")];
    assert_eq!(successors(b"HOH", subs).len(), 4);
    let subs: &[(&[u8], &[u8])] = &[
        (b"e", b"H"),
        (b"e", b"O"),
        (b"H", b"HO"),
        (b"H", b"OH"),
        (b"O", b"HH"),
    ];
}

#[test]
fn test_2() {
    let subs = &[];
    // e -> HF
    assert_eq!(part_2(b"HF", subs), 1);
    // e -> HF -> CRnAlArF
    assert_eq!(part_2(b"CRnAlArF", subs), 2);
    // e -> HF -> CRnFYFYFArF
    assert_eq!(part_2(b"CRnFYFYFArF", subs), 2);
    // e -> HF -> CRnFYFYFArF -> CRnFYFYFArCaF -> CRnFYFYFArSiRnMgArF
    assert_eq!(part_2(b"CRnFYFYFArSiRnMgArF", subs), 4);
}

#[test]
fn test_subseq() {
    assert_eq!(subseq(b"abc", b"abc"), true);
    assert_eq!(subseq(b"ab", b"abc"), false);
    assert_eq!(subseq(b"bc", b"abc"), false);
    assert_eq!(subseq(b"abc", b""), true);
    assert_eq!(subseq(b"abc", b"a"), true);
    assert_eq!(subseq(b"abc", b"c"), true);
    assert_eq!(subseq(b"abc", b"ac"), true);
}
