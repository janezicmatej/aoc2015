use std::cmp::max;

use itertools::Itertools;
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
struct Player {
    hp: i32,
    damage: i32,
    mana: i32,
}

#[derive(Debug, Clone)]
struct Spell {
    name: String,
    timer: i32,
    damage: i32,
    armor: i32,
    heal: i32,
    mana: i32,
    cost: i32,
}

lazy_static! {
    static ref SPELLS: [Spell; 5] = [
        Spell {
            name: "Magic Missile".to_string(),
            timer: 1,
            damage: 4,
            armor: 0,
            heal: 0,
            mana: 0,
            cost: 53,
        },
        Spell {
            name: "Drain".to_string(),
            timer: 1,
            damage: 2,
            armor: 0,
            heal: 2,
            mana: 0,
            cost: 73,
        },
        Spell {
            name: "Shield".to_string(),
            timer: 6,
            damage: 0,
            armor: 7,
            heal: 0,
            mana: 0,
            cost: 113,
        },
        Spell {
            name: "Poison".to_string(),
            timer: 6,
            damage: 3,
            armor: 0,
            heal: 0,
            mana: 0,
            cost: 173,
        },
        Spell {
            name: "Recharge".to_string(),
            timer: 5,
            damage: 0,
            armor: 0,
            heal: 0,
            mana: 101,
            cost: 229,
        },
    ];
}

fn optimize_fight(
    mut boss: Player,
    mut player: Player,
    players_turn: bool,
    mut active_spells: Vec<Spell>,
    best_cost: &mut i32,
    cost: i32,
    hardmode: bool,
) {
    if cost > *best_cost {
        return;
    }
    let mut remaining = active_spells
        .drain(..)
        .filter(|x| x.timer > 0)
        .collect_vec();
    let mut armor = 0;
    for spell in remaining.iter_mut() {
        spell.timer -= 1;
        boss.hp -= spell.damage;
        armor += spell.armor;
        player.hp += spell.heal;
        player.mana += spell.mana;
    }

    if hardmode && players_turn {
        player.hp -= 1;
    }
    if player.hp <= 0 {
        return;
    }
    if boss.hp <= 0 {
        if cost < *best_cost {
            *best_cost = cost;
        }
        return;
    }

    if players_turn {
        let current = remaining
            .iter()
            .filter(|x| x.timer > 0)
            .map(|x| x.name.to_string())
            .collect_vec();
        for spell in SPELLS
            .iter()
            .filter(|x| !current.contains(&x.name) && x.cost < player.mana)
            .collect_vec()
        {
            let mut new_spells = remaining.to_vec();
            new_spells.push(spell.clone());
            let mut c = player;
            c.mana -= spell.cost;
            optimize_fight(
                boss,
                c,
                !players_turn,
                new_spells,
                best_cost,
                cost + spell.cost,
                hardmode,
            );
        }
    } else {
        let damage = max(boss.damage - armor, 1);
        player.hp -= damage;
        optimize_fight(
            boss,
            player,
            !players_turn,
            remaining,
            best_cost,
            cost,
            hardmode,
        );
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let (hp, damage) = input
        .split('\n')
        .map(|x| x.split(": ").last().unwrap().parse::<i32>().unwrap())
        .next_tuple()
        .unwrap();
    let boss = Player {
        hp,
        damage,
        mana: 0,
    };
    let me = Player {
        hp: 50,
        damage: 0,
        mana: 500,
    };
    // this takes 7minutes but iiwii
    let mut best_score = i32::max_value();
    optimize_fight(boss, me, true, vec![], &mut best_score, 0, false);
    Some(best_score)
}
pub fn part_two(input: &str) -> Option<i32> {
    let (hp, damage) = input
        .split('\n')
        .map(|x| x.split(": ").last().unwrap().parse::<i32>().unwrap())
        .next_tuple()
        .unwrap();
    let boss = Player {
        hp,
        damage,
        mana: 0,
    };
    let me = Player {
        hp: 50,
        damage: 0,
        mana: 500,
    };
    let mut best_score = i32::max_value();
    optimize_fight(boss, me, true, vec![], &mut best_score, 0, true);
    Some(best_score)
}
fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = aoc::read_file("test_inputs", 22);
        assert_eq!(part_one(input.trim()), Some(212));
    }
    #[test]
    fn test_part_two() {
        let input = aoc::read_file("test_inputs", 22);
        assert_eq!(part_two(input.trim()), Some(212));
    }
}
