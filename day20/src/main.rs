extern crate factor;

use factor::factor_include::factor_include;

fn main() {
    println!("{:?}", part_1(36000000));
    println!("{:?}", part_2(36000000));
}

fn part_1(min: usize) -> usize {
    let mut max_house = 0;
    for i in 0..20 {
        let fac = (1..i).product::<usize>();
        let gifts = factor_include(fac as i64)
            .iter()
            .map(|elf| elf * 10)
            .sum::<i64>() as usize;
        if gifts > min {
            max_house = fac;
            break;
        }
    }
    println!("max house: {}", max_house);
    let mut gifts = vec![0; max_house + 1];
    for elf in 1..=max_house {
        for h in 1..=max_house / elf {
            gifts[h * elf] += 10 * elf;
        }
    }
    gifts.iter().skip(1).position(|&h| h >= min).unwrap() + 1
}

fn part_2(min: usize) -> usize {
    let max_house = min;
    println!("max house: {}", max_house);
    let mut gifts = vec![0; max_house + 1];
    for elf in 1..=max_house {
        for h in 1..=(max_house / elf).min(50) {
            gifts[h * elf] += 11 * elf;
        }
    }
    gifts.iter().skip(1).position(|&h| h >= min).unwrap() + 1
}
