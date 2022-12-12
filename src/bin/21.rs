use itertools::Itertools;
use ItemType::*;

static STORE: [Item; 16] = [
    // weapons
    Item {
        item_type: Weapon,
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        item_type: Weapon,
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        item_type: Weapon,
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        item_type: Weapon,
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        item_type: Weapon,
        cost: 74,
        damage: 8,
        armor: 0,
    },
    // armor
    Item {
        item_type: Armor,
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        item_type: Armor,
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        item_type: Armor,
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        item_type: Armor,
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        item_type: Armor,
        cost: 102,
        damage: 0,
        armor: 5,
    },
    // rings
    Item {
        item_type: Ring,
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        item_type: Ring,
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        item_type: Ring,
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        item_type: Ring,
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        item_type: Ring,
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        item_type: Ring,
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Clone, Copy)]
struct Player {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Player {
    fn alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Debug)]
enum ItemType {
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug)]
struct Item {
    item_type: ItemType,
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn valid_selection(selection: &[&Item]) -> bool {
        if selection
            .iter()
            .filter(|x| matches!(x.item_type, Weapon))
            .count()
            != 1
        {
            return false;
        }

        if selection
            .iter()
            .filter(|x| matches!(x.item_type, Armor))
            .count()
            > 1
        {
            return false;
        }

        if selection
            .iter()
            .filter(|x| matches!(x.item_type, Ring))
            .count()
            > 2
        {
            return false;
        }

        true
    }
}

fn simulate(me: Player, items: &[&Item], mut boss: Player) -> bool {
    let mut equiped_me = Player {
        hp: me.hp,
        damage: me.damage + items.iter().map(|x| x.damage).sum::<i32>(),
        armor: me.armor + items.iter().map(|x| x.armor).sum::<i32>(),
    };

    while equiped_me.alive() && boss.alive() {
        let mut my_damage = equiped_me.damage - boss.armor;
        if my_damage < 1 {
            my_damage = 1;
        }

        let mut boss_damage = boss.damage - equiped_me.armor;
        if boss_damage < 1 {
            boss_damage = 1;
        }

        equiped_me.hp -= boss_damage;
        boss.hp -= my_damage;
    }

    !boss.alive()
}
pub fn part_one(input: &str) -> Option<i32> {
    let (hp, damage, armor) = input
        .split('\n')
        .map(|x| x.split(": ").last().unwrap().parse::<i32>().unwrap())
        .next_tuple()
        .unwrap();
    let boss = Player { hp, damage, armor };
    let me = Player {
        hp: 100,
        damage: 0,
        armor: 0,
    };

    (0..STORE.len())
        .map(|x| {
            STORE
                .iter()
                .combinations(x)
                .filter(|y| Item::valid_selection(y))
                .map(|y| {
                    (
                        y.iter().map(|z| z.cost).sum::<i32>(),
                        simulate(me, &y, boss),
                    )
                })
                .filter(|(_, y)| *y)
                .map(|(y, _)| y)
                .collect_vec()
        })
        .filter_map(|x| x.iter().min().copied())
        .min()
}
pub fn part_two(input: &str) -> Option<i32> {
    let (hp, damage, armor) = input
        .split('\n')
        .map(|x| x.split(": ").last().unwrap().parse::<i32>().unwrap())
        .next_tuple()
        .unwrap();
    let boss = Player { hp, damage, armor };
    let me = Player {
        hp: 100,
        damage: 0,
        armor: 0,
    };

    (0..STORE.len())
        .map(|x| {
            STORE
                .iter()
                .combinations(x)
                .filter(|y| Item::valid_selection(y))
                .map(|y| {
                    (
                        y.iter().map(|z| z.cost).sum::<i32>(),
                        !simulate(me, &y, boss),
                    )
                })
                .filter(|(_, y)| *y)
                .map(|(y, _)| y)
                .collect_vec()
        })
        .filter_map(|x| x.iter().max().copied())
        .max()
}
fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}
