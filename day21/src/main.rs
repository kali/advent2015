extern crate itertools;

use itertools::Itertools;

fn main() {
    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors = [
        (0, 0, 0),
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
    ];
    let rings = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ];
    let mut cheapest_winner = usize::max_value();
    let mut most_exp_looser = 0;
    for w in &weapons {
        for a in &armors {
            for ring_count in 0..=2 {
                for rings in rings.iter().combinations(ring_count) {
                    let cost = w.0 + a.0 + rings.iter().map(|r| r.0).sum::<usize>();
                    let dam = w.1 + a.1 + rings.iter().map(|r| r.1).sum::<usize>();
                    let armor = w.2 + a.2 + rings.iter().map(|r| r.2).sum::<usize>();
                    let mut player = Fighter { hp: 100, armor, dam };
                    let mut fighter = Fighter { hp: 103, armor: 2, dam: 9};
                    fight(&mut player, &mut fighter);
                    if player.hp > 0 {
                        cheapest_winner = cheapest_winner.min(cost);
                    } else {
                        most_exp_looser = most_exp_looser.max(cost);
                    }
                }
            }
        }
    }
    println!("{}", cheapest_winner);
    println!("{}", most_exp_looser);
}

struct Fighter {
    hp: usize,
    dam: usize,
    armor: usize,
}

fn fight(player: &mut Fighter, boss: &mut Fighter) {
    loop {
        boss.hp = boss
            .hp
            .saturating_sub(player.dam.saturating_sub(boss.armor).max(1));
        if boss.hp == 0 {
            return;
        }
        player.hp = player
            .hp
            .saturating_sub(boss.dam.saturating_sub(player.armor).max(1));
        if player.hp == 0 {
            return;
        }
    }
}

#[test]
fn t_fight() {
    let mut player = Fighter {
        hp: 8,
        dam: 5,
        armor: 5,
    };
    let mut boss = Fighter {
        hp: 12,
        dam: 7,
        armor: 2,
    };
    fight(&mut player, &mut boss);
    assert_eq!(player.hp, 2);
    assert_eq!(boss.hp, 0);
}
