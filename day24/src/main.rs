use itertools::Itertools;
use std::fs;

fn can_split_rest(rest: &[usize], groups: usize, group_mass: usize) -> bool {
    groups == 1
        || (1..rest.len()).any(|n| {
            rest.iter().rev().take(n).cloned().sum::<usize>() >= group_mass
                && rest.iter().take(n).cloned().sum::<usize>() <= group_mass
                && rest.iter().combinations(n).any(|comb| {
                    comb.iter().copied().sum::<usize>() == group_mass && {
                        let rest: Vec<usize> = rest
                            .iter()
                            .copied()
                            .filter(|x| !comb.contains(&x))
                            .collect();
                        can_split_rest(&*rest, groups - 1, group_mass)
                    }
                })
        })
}

fn main() {
    let packs: Vec<usize> = fs::read_to_string("input")
        .unwrap()
        .lines()
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let mass = packs.iter().cloned().sum::<usize>();
    for &groups in &[3, 4] {
        let group_mass = mass / groups;
        for n in 1..packs.len() {
            if packs.iter().rev().take(n).cloned().sum::<usize>() < group_mass {
                continue;
            }
            println!("looking for combinations for n={}", n);
            let combs: Vec<Vec<usize>> = packs
                .iter()
                .combinations(n)
                .filter(|comb| comb.iter().cloned().sum::<usize>() == group_mass)
                .filter(|comb| {
                    let rest: Vec<usize> = packs
                        .iter()
                        .copied()
                        .filter(|x| !comb.contains(&x))
                        .collect();
                    can_split_rest(&*rest, groups - 1, group_mass)
                })
                .map(|comb| comb.iter().cloned().cloned().collect::<Vec<usize>>())
                .collect();
            if combs.len() > 0 {
                let min_qe = combs
                    .iter()
                    .map(|comb| comb.iter().product::<usize>())
                    .min();
                dbg!(min_qe);
                break;
            }
        }
    }
}
