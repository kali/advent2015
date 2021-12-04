use std::iter::once;

fn main() {
    let s = std::fs::read_to_string("input").unwrap();
    println!("{:?}", run(s.trim(), 100, 100, false));
    println!("{:?}", run(s.trim(), 100, 100, true));
}

fn run(s: &str, side: isize, step: usize, second_part: bool) -> usize {
    let mut map = vec![vec![false; side as usize + 2]];
    s.split("\n").for_each(|s| {
        map.push(
            once(false)
                .chain(s.bytes().map(|c| c == b'#'))
                .chain(once(false))
                .collect(),
        )
    });
    map.push(vec![false; side as usize + 2]);
    if second_part {
        map[1][1] = true;
        map[1][side as usize] = true;
        map[side as usize][1] = true;
        map[side as usize][side as usize] = true;
    }
    for _ in 0..step {
        /*
        for l in &map {
            println!("{}", l.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>());
        }
        println!("");
        */
        let mut next = vec![vec![false; side as usize + 2]; side as usize + 2];
        for x in 1isize..=side {
            for y in 1isize..=side {
                let ons = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ]
                .iter()
                .filter(|&(dx, dy)| map[(y + dy) as usize][(x + dx) as usize])
                .count();
                next[y as usize][x as usize] = if map[y as usize][x as usize] {
                    ons == 2 || ons == 3
                } else {
                    ons == 3
                };
            }
        }
        map = next;
        if second_part {
            map[1][1] = true;
            map[1][side as usize] = true;
            map[side as usize][1] = true;
            map[side as usize][side as usize] = true;
        }
    }
    map.iter().map(|l| l.iter().filter(|&x| *x).count()).sum::<usize>()
}

#[test]
fn test() {
    let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
    assert_eq!(run(input, 6, 4, false), 4);
    assert_eq!(run(input, 6, 5, true), 17);
}
