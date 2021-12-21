use std::cmp::{max, min};
use std::collections::HashMap;

// Player 1 starting position: 10
const START_P1: u8 = 10;

// Player 2 starting position: 3
const START_P2: u8 = 3;

struct World {
    cache: HashMap<(u8, u8, u8, u8, u8), (u64, u64)>,
}

impl World {
    fn add(&mut self, key: (u8, u8, u8, u8, u8), value: (u64, u64)) {
        self.cache
            .entry(key)
            .and_modify(|i| *i = (i.0 + value.0, i.1 + value.1))
            .or_insert(value);
    }

    fn simulate(&mut self, key: (u8, u8, u8, u8, u8)) -> (u64, u64) {
        /*
        pos_p1: u8,
        score_p1: u8,
        pos_p2: u8,
        score_p2: u8,
        which: u8,*/

        // If cached, return it
        if let Some(cached) = self.cache.get(&key) {
            return *cached;
        }

        // Now let's try assuming that this player might role
        for d1 in [1, 2, 3] {
            for d2 in [1, 2, 3] {
                for d3 in [1, 2, 3] {
                    // Simulate player 1 if it's their turn
                    if key.4 == 1 {
                        let new_pos_p1 = ((((key.0 as u32 - 1) + (d1 + d2 + d3)) % 10) + 1) as u8;
                        let new_score_p1 = key.1 + new_pos_p1;
                        if new_score_p1 >= 21 {
                            // Player 1 wins in this state
                            self.add(key, (1, 0));
                        } else {
                            // No win, continue game from player 2's position and add that to
                            // our own winning cache
                            let new_key = (new_pos_p1, new_score_p1, key.2, key.3, 2);
                            let rv = self.simulate(new_key);
                            self.add(key, rv);
                        }
                    }

                    // Simulate player 2
                    if key.4 == 2 {
                        let new_pos_p2 = ((((key.2 as u32 - 1) + (d1 + d2 + d3)) % 10) + 1) as u8;
                        let new_score_p2 = key.3 + new_pos_p2;
                        if new_score_p2 >= 21 {
                            // Player 1 wins in this state
                            self.add(key, (0, 1));
                        } else {
                            // No win, continue game from player 2's position and add that to
                            // our own winning cache
                            let new_key = (key.0, key.1, new_pos_p2, new_score_p2, 1);
                            let rv = self.simulate(new_key);
                            self.add(key, rv);
                        }
                    }
                }
            }
        }

        let rv = *self.cache.get(&key).unwrap();
        //dbg!(key, rv);
        return rv;
    }
}

fn part_one() -> u32 {
    let mut rolls = 0;
    let mut pos_p1 = START_P1;
    let mut score_p1 = 0;
    let mut pos_p2 = START_P2;
    let mut score_p2 = 0;

    loop {
        //dbg!(rolls, pos_p1, score_p1, pos_p2, score_p2);
        pos_p1 = ((((pos_p1 - 1) as u32 + (rolls + 2u32) * 3u32) % 10) + 1) as u8;
        rolls += 3;
        score_p1 += pos_p1 as u32;
        if score_p1 >= 1000 {
            break;
        }

        pos_p2 = ((((pos_p2 - 1) as u32 + (rolls + 2u32) * 3u32) % 10) + 1) as u8;
        rolls += 3;
        score_p2 += pos_p2 as u32;
        if score_p2 >= 1000 {
            break;
        }
        //dbg!(rolls, pos_p1, score_p1, pos_p2, score_p2);
    }

    rolls as u32 * min(score_p1, score_p2) as u32
}

fn part_two() -> u64 {
    // Given starting positions, start by calculating who wins based on
    // each universe state, attempt to memoize??

    let mut world = World {
        cache: HashMap::new(),
    };

    let rv = world.simulate((START_P1, 0, START_P2, 0, 1));
    max(rv.0, rv.1)
}

fn main() {
    println!("PART ONE: {}", part_one());
    println!("PART TWO: {}", part_two());
}
