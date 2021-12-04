#[derive(Copy, Clone, Debug, PartialEq)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn all() -> &'static [Spell] {
        use crate::Spell::*;
        &[Missile, Drain, Shield, Poison, Recharge]
    }
    fn cost(&self) -> usize {
        use crate::Spell::*;
        match self {
            Missile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
struct State {
    turn: usize,
    player_hp: usize,
    player_mana: usize,
    boss_hp: usize,
    boss_damage: usize,
    poison_end: usize,
    recharge_end: usize,
    shield_end: usize,
}

impl State {
    fn new(player_hp: usize, player_mana: usize, boss_hp: usize, boss_damage: usize) -> State {
        State {
            player_mana,
            player_hp,
            boss_hp,
            boss_damage,
            ..State::default()
        }
    }

    fn tick(&self) -> State {
        State {
            turn: self.turn + 1,
            boss_hp: self
                .boss_hp
                .saturating_sub((self.poison_end > self.turn) as usize * 3),
            player_mana: self.player_mana + (self.recharge_end > self.turn) as usize * 101,
            ..*self
        }
    }

    fn player_turn(&self, s: Spell, hard: bool) -> State {
        use crate::Spell::*;
        let mut state = self.clone();
        if hard {
            state.player_hp = state.player_hp.saturating_sub(1);
        }
        if state.finished() {
            return state;
        }
        state.player_mana -= s.cost();
        match s {
            Missile => {
                state.boss_hp = state.boss_hp.saturating_sub(4);
            }
            Drain => {
                state.boss_hp = state.boss_hp.saturating_sub(2);
                state.player_hp += 2;
            }
            Shield => {
                state.shield_end = state.turn + 6;
            }
            Poison => {
                state.poison_end = state.turn + 6;
            }
            Recharge => {
                state.recharge_end = state.turn + 5;
            }
        }
        state
    }

    fn boss_turn(&self) -> State {
        let mut state = self.clone();
        if state.finished() {
            return state;
        }
        let damage = state
            .boss_damage
            .saturating_sub((state.shield_end > state.turn) as usize * 7)
            .max(1);
        state.player_hp = state.player_hp.saturating_sub(damage);
        state
    }

    fn finished(&self) -> bool {
        self.player_hp == 0 || self.boss_hp == 0
    }

    fn won(&self) -> bool {
        self.player_hp > 0 || self.boss_hp == 0
    }
}

fn main() {
    println!("{}", run(false));
    println!("{}", run(true));
}

fn run(hard:bool) -> usize {
    let mut s = State::new(50, 500, 55, 8);
    s.turn += 1;
    let result = pathfinding::directed::dijkstra::dijkstra(
        &s,
        |state| {
            use crate::Spell::*;
            let state:State = state.clone();
            let mana = state.player_mana;
            let turn = state.turn;
            Spell::all()
                .iter()
                .filter(move |spell| spell.cost() <= mana)
                .filter(move |spell| match spell {
                    Missile | Drain => true,
                    Shield => state.shield_end <= turn,
                    Recharge => state.recharge_end <= turn,
                    Poison => state.poison_end <= turn,
                })
                .map(move |&spell| {
                    let mut state = state.player_turn(spell, hard);
                    if !state.finished() {
                        state = state.tick();
                    }
                    if !state.finished() {
                        state = state.boss_turn();
                    }
                    if !state.finished() {
                        state = state.tick();
                    }
                    (state, spell.cost())
                })
        },
        |state| state.boss_hp == 0,
    );
    result.unwrap().1
}

#[test]
fn t1() {
    use crate::Spell::*;
    let mut s = State::new(10, 250, 13, 8);
    s = s.tick().player_turn(Poison, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (10, 77, 13));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 77, 10));
    s = s.tick().player_turn(Missile, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 24, 3));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 24, 0));
}

#[test]
fn t2() {
    use crate::Spell::*;
    let mut s = State::new(10, 250, 14, 8);
    s = s.tick().player_turn(Recharge, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (10, 21, 14));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 122, 14));
    s = s.tick().player_turn(Shield, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 110, 14));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (1, 211, 14));
    s = s.tick().player_turn(Drain, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (3, 239, 12));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 340, 12));
    s = s.tick().player_turn(Poison, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (2, 167, 12));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (1, 167, 9));
    s = s.tick().player_turn(Missile, false);
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (1, 114, 2));
    s = s.tick().boss_turn();
    assert_eq!((s.player_hp, s.player_mana, s.boss_hp), (1, 114, 0));
}
