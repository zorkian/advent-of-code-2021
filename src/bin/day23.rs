#![allow(dead_code, unused_variables)]

use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Pod {
    energy: u32,
    homeroom: usize,
}

#[derive(Debug, Clone, Copy)]
struct Room {
    id: usize,
    top: Option<Pod>,
    bottom: Option<Pod>,
}

#[derive(Debug, Clone, Copy)]
struct World {
    hallway: [Option<Pod>; 11],
    room: [Room; 4],
}

impl Pod {
    fn new(energy: u32, homeroom: usize) -> Pod {
        return Pod { energy, homeroom };
    }

    fn char(&self) -> &str {
        match self.energy {
            1 => "A",
            10 => "B",
            100 => "C",
            1000 => "D",
            _ => panic!("unknown"),
        }
    }
}

impl Room {
    fn accepting_pods(&self) -> bool {
        // True if there's an open space for the pod that should be here
        (self.top.is_none() && self.bottom.is_none())
            || (self.top.is_none() && self.bottom.unwrap().homeroom == self.id)
    }

    fn happy(&self) -> bool {
        (self.top.is_some() && self.top.unwrap().homeroom == self.id)
            && (self.bottom.is_some() && self.bottom.unwrap().homeroom == self.id)
    }

    fn position(&self) -> usize {
        // Which hallway position we're under
        (self.id + 1) * 2
    }
}

impl World {
    fn state(&self) -> String {
        // Return state string for this
        let mut rv = Vec::new();

        for idx in 0..self.hallway.len() {
            if let Some(pod) = self.hallway[idx] {
                rv.push(format!("{}H{}", pod.char(), idx));
            }
        }

        for idx in 0..self.room.len() {
            if let Some(pod) = self.room[idx].top {
                rv.push(format!("{}T{}", pod.char(), idx));
            }
            if let Some(pod) = self.room[idx].bottom {
                rv.push(format!("{}B{}", pod.char(), idx));
            }
        }

        // rv.sort();
        rv.join("")
    }

    fn hallway_is_clear(&self, from: usize, to: usize, allow_from: bool) -> bool {
        for idx in min(from, to)..=max(from, to) {
            if self.hallway[idx].is_some() {
                if allow_from && (idx == from) {
                    continue;
                }
                return false;
            }
        }
        true
    }

    fn valid_hallway(&self, room_pos: usize) -> Vec<usize> {
        // Return set of valid hallway positions from a given room location
        let mut rv = Vec::new();

        for idx in room_pos + 1..11 {
            if self.hallway[idx].is_some() {
                break;
            }
            if idx == 2 || idx == 4 || idx == 6 || idx == 8 {
                continue;
            }
            rv.push(idx);
        }
        for idx in (0..room_pos).rev() {
            if self.hallway[idx].is_some() {
                break;
            }
            if idx == 2 || idx == 4 || idx == 6 || idx == 8 {
                continue;
            }
            rv.push(idx);
        }
        rv
    }

    fn print(&self) {
        println!("#############");
        print!("#");
        for idx in 0..self.hallway.len() {
            if let Some(pod) = self.hallway[idx] {
                print!("{}", pod.char());
            } else {
                print!(".");
            }
        }
        print!("#\n###");
        for idx in 0..self.room.len() {
            if let Some(pod) = self.room[idx].top {
                print!("{}#", pod.char());
            } else {
                print!(".#");
            }
        }
        print!("##\n  #");
        for idx in 0..self.room.len() {
            if let Some(pod) = self.room[idx].bottom {
                print!("{}#", pod.char());
            } else {
                print!(".#");
            }
        }
        println!("\n  #########");
    }

    fn solve(
        &self,
        in_score: u32,
        events: &mut Vec<String>,
        cache: &mut HashMap<String, Option<u32>>,
    ) -> Option<u32> {
        // From wherever the world state is, calculate the minimal cost based on the
        // potential moves we could do to solve it
        //
        // 1. If there's someone in the hallway who can get home, without blocking the
        //    room such that they'd have to leave again, send them home
        // 2. If there is a pod that can get out of its room and make it straight into
        //    its homeroom and not block anybody in, send them home (two steps)
        // 3. If there is a pod on top that is blocking in someone below it, move this
        //    pod out and *away* from the other pod's home room by a minimal but increasing
        //    distance
        // 4. Repeat?

        let mut scores = Vec::new();

        // If this state is in our cache, return it
        let mut events_sorted = events.clone();
        events_sorted.sort();
        let cache_key = events_sorted.join("");

        if let Some(cached) = cache.get(&cache_key) {
            // println!("Cache hit: {} = {:?}", cache_key, cached);
            return *cached;
        }

        // Base case, if everybody is home...
        let mut happy = 0;
        for idx in 0..self.room.len() {
            if self.room[idx].happy() {
                happy += 1;
            }
        }
        if happy == self.room.len() {
            if in_score <= 12521 {
                self.print();
                let last_event = events.pop().unwrap();
                events.push(format!("{}, solved with {} points", last_event, in_score));
                dbg!(events);
            }
            return Some(in_score);
        }

        let mut world = self.clone();
        let mut score = |world: &World,
                         events: &mut Vec<String>,
                         cache: &mut HashMap<String, Option<u32>>,
                         reason: String,
                         score: u32| {
            // println!("Scored {} after {}", score, reason);
            events.push(reason); // format!("{}, score {}", reason, score));
            if let Some(total_score) = world.solve(score + in_score, events, cache) {
                scores.push(total_score);
            }
            events.pop();
        };

        // 1. Hallway pod -> room
        for idx in 0..world.hallway.len() {
            let mut rv = 0;
            if let Some(pod) = world.hallway[idx] {
                // See if the hallway is clear from here to there
                let hallway_clear =
                    world.hallway_is_clear(idx, world.room[pod.homeroom].position(), true);

                // See if this pod can go to its home room
                if hallway_clear && world.room[pod.homeroom].accepting_pods() {
                    // Yes, calculate the delta to move this pod to its location and then do
                    // so, this is guaranteed to be a minimum cost move since we don't have to
                    // do any further movement out of this hallway
                    //
                    // The variable part of the cost is paid *previously* by when we move into
                    // the hallway, so it's safe to fast-path this
                    let mut distance =
                        (world.room[pod.homeroom].position() as i32 - idx as i32).abs() as u32;
                    if world.room[pod.homeroom].bottom.is_none() {
                        distance += 2;
                        world.room[pod.homeroom].bottom = Some(pod);
                    } else {
                        distance += 1;
                        world.room[pod.homeroom].top = Some(pod);
                    };
                    rv += pod.energy * distance;
                    world.hallway[idx] = None;

                    // Now cascade this down to calculate the minimal score from our new world
                    // state, based on this one move we did
                    score(
                        &world,
                        events,
                        cache,
                        format!("MPH{}{}{}", pod.char(), idx, pod.homeroom),
                        rv,
                    );
                    world = self.clone();
                }
            }
        }

        // Now for each room, consider that we could try sending a pod home directly if
        // we have a clear path (guaranteed efficient), or if we should move a pod out of
        // the way
        for idx in 0..world.room.len() {
            let room = world.room[idx];

            if let Some(pod) = room.top {
                // Top pod exists, if it's not home, see if we can send it home
                if pod.homeroom == room.id {
                    continue;
                }

                // TODO: Pod is not home, try
                //   a. Send it straight home (room in the target)
                let target_room = self.room[pod.homeroom];

                // See if the hallway is clear from here to there
                let hallway_clear =
                    world.hallway_is_clear(room.position(), target_room.position(), false);

                if hallway_clear && target_room.accepting_pods() {
                    // Yes we can go straight there, calculate it
                    let mut distance =
                        2 + (room.position() as i32 - target_room.position() as i32).abs();
                    if target_room.bottom.is_none() {
                        // If the bottom is open, use it (we know the top is open, since the target
                        // room is accepting pods)
                        distance += 1;
                        world.room[pod.homeroom].bottom = Some(pod);
                    } else {
                        world.room[pod.homeroom].top = Some(pod);
                    }

                    // Actually move the pod in our world, score it, then reset
                    world.room[idx].top = None;
                    score(
                        &world,
                        events,
                        cache,
                        format!("MTP2{}{}{}", distance, idx, pod.homeroom),
                        pod.energy * distance as u32,
                    );
                    world = self.clone();
                }
            } else if let Some(pod) = room.bottom {
                // No top pod, only bottom, so try
                if pod.homeroom == room.id {
                    continue;
                }

                // TODO: Pod is not home, try
                //   a. Send it straight home (room in the target)
                let target_room = self.room[pod.homeroom];

                // See if the hallway is clear from here to there
                let hallway_clear =
                    world.hallway_is_clear(room.position(), target_room.position(), false);

                if hallway_clear && target_room.accepting_pods() {
                    // Yes we can go straight there, calculate it
                    let mut distance =
                        3 + (room.position() as i32 - target_room.position() as i32).abs();
                    if target_room.bottom.is_none() {
                        // If the bottom is open, use it (we know the top is open, since the target
                        // room is accepting pods)
                        distance += 1;
                        world.room[pod.homeroom].bottom = Some(pod);
                    } else {
                        world.room[pod.homeroom].top = Some(pod);
                    }

                    // Actually move the pod in our world, score it, then reset
                    world.room[idx].bottom = None;
                    score(
                        &world,
                        events,
                        cache,
                        format!("MBP{}{}{}{}", pod.char(), distance, idx, pod.homeroom),
                        pod.energy * distance as u32,
                    );
                    world = self.clone();
                }
            }
        }

        // We weren't able to send anybody directly home, so try moving things out into the hallway
        // one at a time, one position at a time, and hope that this lets us solve things
        for idx in 0..world.room.len() {
            let room = world.room[idx];

            if let Some(pod) = room.top {
                if pod.homeroom == room.id {
                    // However, if the bottom pod is not home, then we should move
                    if let Some(bpod) = room.bottom {
                        if bpod.homeroom == room.id {
                            continue;
                        }
                    }
                }
                for hallway_idx in world.valid_hallway(room.position()) {
                    // Attempt to move to this position and solve from there
                    let distance = 1 + (room.position() as i32 - hallway_idx as i32).abs();
                    world.hallway[hallway_idx] = Some(pod);
                    world.room[idx].top = None;
                    score(
                        &world,
                        events,
                        cache,
                        format!("MTP{}{}{}", pod.char(), idx, hallway_idx),
                        pod.energy * distance as u32,
                    );
                    world = self.clone();
                }
            } else if let Some(pod) = room.bottom {
                if pod.homeroom == room.id {
                    continue;
                }
                for hallway_idx in world.valid_hallway(room.position()) {
                    // Attempt to move to this position and solve from there
                    let distance = 2 + (room.position() as i32 - hallway_idx as i32).abs();
                    world.hallway[hallway_idx] = Some(pod);
                    world.room[idx].bottom = None;
                    score(
                        &world,
                        events,
                        cache,
                        format!("MBP{}{}{}", pod.char(), idx, hallway_idx),
                        pod.energy * distance as u32,
                    );
                    world = self.clone();
                }
            }
        }

        if scores.len() == 0 {
            cache.insert(cache_key, None);
            return None;
        }
        let smin = scores.iter().map(|i| *i).min();
        cache.insert(cache_key, smin);
        smin
    }
}

fn part_one(world: &World) -> u32 {
    if let Some(score) = world.solve(0, &mut Vec::new(), &mut HashMap::new()) {
        return score;
    }
    panic!("no score");
}

fn part_two(world: &World) -> u32 {
    0
}

fn main() {
    let amber = Pod::new(1, 0);
    let bronze = Pod::new(10, 1);
    let copper = Pod::new(100, 2);
    let desert = Pod::new(1000, 3);

    let world = World {
        hallway: [None; 11],
        room: [
            Room {
                id: 0,
                top: Some(desert),
                bottom: Some(desert),
            },
            Room {
                id: 1,
                top: Some(amber),
                bottom: Some(copper),
            },
            Room {
                id: 2,
                top: Some(copper),
                bottom: Some(bronze),
            },
            Room {
                id: 3,
                top: Some(amber),
                bottom: Some(bronze),
            },
        ],
    };

    let world = World {
        hallway: [None; 11],
        room: [
            Room {
                id: 0,
                top: Some(bronze),
                bottom: Some(amber),
            },
            Room {
                id: 1,
                top: Some(copper),
                bottom: Some(desert),
            },
            Room {
                id: 2,
                top: Some(bronze),
                bottom: Some(copper),
            },
            Room {
                id: 3,
                top: Some(desert),
                bottom: Some(amber),
            },
        ],
    };

    world.print();

    println!("PART ONE: {}", part_one(&world));
    println!("PART TWO: {}", part_two(&world));
}
