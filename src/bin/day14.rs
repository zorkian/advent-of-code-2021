use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug)]
struct Insertion {
    pair: [char; 2],
    insert: char,
    counter: u64,
    is_tail: bool,
}

type Insertions = HashMap<[char; 2], Insertion>;

impl Insertion {
    fn new(input: &str) -> Insertion {
        let parts: Vec<&str> = input.split(" ").collect();

        let mut chars: [char; 2] = [' '; 2];
        let initial: Vec<char> = parts[0].to_string().chars().collect();
        chars[0] = initial[0];
        chars[1] = initial[1];

        Insertion {
            pair: chars,
            insert: parts[2].to_string().chars().next().unwrap(),
            counter: 0,
            is_tail: false,
        }
    }
}

fn print_insertions(insertions: &Insertions) {
    println!("---");
    for insertion in insertions.values() {
        println!(
            "{}{} -> {} ({}, {})",
            insertion.pair[0],
            insertion.pair[1],
            insertion.insert,
            insertion.counter,
            insertion.is_tail
        );
    }
}

fn make_insertions(input: String, insertions: &Insertions) -> String {
    let mut pair: [char; 2] = [' '; 2];
    let mut rv = "".to_string();

    for (idx, char) in input.chars().enumerate() {
        if idx == 0 {
            pair[0] = char;
            continue;
        } else if idx == 1 {
            pair[1] = char;
        } else {
            pair[0] = pair[1];
            pair[1] = char;
        }

        let mut insert: char = ' ';
        if let Some(insertion) = insertions.get(&pair) {
            insert = insertion.insert;
        }
        rv += &pair[0].to_string();
        if insert != ' ' {
            rv += &insert.to_string();
        }
    }
    rv += &pair[1].to_string();

    rv
}

fn make_insertions_faster(insertions: &Insertions) -> Insertions {
    let mut rv = Insertions::new();

    // At the very least, our destination looks like our source, so copy
    // of everything we know about
    for insertion in insertions.values() {
        rv.entry(insertion.pair).or_insert(Insertion {
            pair: insertion.pair,
            insert: insertion.insert,
            counter: if insertion.insert == ' ' {
                insertion.counter
            } else {
                0
            },
            is_tail: false,
        });
    }

    // Now the only thing we need to do is handle creating insertions
    // which is based on the rightmost pair
    for insertion in insertions.values() {
        if insertion.counter == 0 || insertion.insert == ' ' {
            // Not a valid insertion, the counter was already preserved in the
            // initialization above, but we need to keep the tail value just
            // in case
            rv.entry(insertion.pair)
                .and_modify(|i| i.is_tail |= insertion.is_tail);
            continue;
        }

        // We need to transfer our count to the leftmost pair that is created
        // from our coupling
        let left_pair: [char; 2] = [insertion.pair[0], insertion.insert];
        let mut entry = rv.entry(left_pair).or_insert(Insertion {
            pair: left_pair,
            insert: ' ',
            counter: 0,
            is_tail: false,
        });
        entry.counter += insertion.counter;
        /*println!(
            "(L) {:?} += {} = {} (was tail {})",
            left_pair, insertion.counter, entry.counter, insertion.is_tail
        );*/

        // In essence, we are adding the count of our original pair to the
        // count of the rightmost pair, since we only count the letter on the
        // left side of the pair, this in essence adds copies of the inserted
        // letter (which is the left side of the right pair)
        let right_pair: [char; 2] = [insertion.insert, insertion.pair[1]];
        let mut entry = rv.entry(right_pair).or_insert(Insertion {
            pair: right_pair,
            insert: ' ',
            counter: 0,
            is_tail: false,
        });
        entry.counter += insertion.counter;
        entry.is_tail |= insertion.is_tail;
        /*println!(
            "(R) {:?} += {} = {} (was tail {}, is tail {})",
            right_pair, insertion.counter, entry.counter, insertion.is_tail, entry.is_tail
        );*/
    }
    rv
}

fn parse_input(input: &str) -> (String, Insertions) {
    let mut initial = "";
    let mut insertions = Insertions::new();

    for line in input.split("\n") {
        if initial == "" {
            initial = line;
            continue;
        } else if line == "" {
            continue;
        }
        let insertion = Insertion::new(line);
        insertions.insert(insertion.pair, insertion);
    }

    // Now parse the initial line into insertion counts so that we can do
    // the non-naive version
    let chars: Vec<char> = initial.chars().collect();
    for idx in 1..chars.len() {
        let pair: [char; 2] = [chars[idx - 1], chars[idx]];
        let entry = insertions.entry(pair).or_insert(Insertion {
            pair,
            insert: ' ',
            counter: 0,
            is_tail: false,
        });
        entry.counter += 1;
        entry.is_tail = idx == (chars.len() - 1);
    }

    (initial.to_string(), insertions)
}

fn part_one(input: &str) -> u32 {
    let (mut initial, insertions) = parse_input(input);

    for _ in 0..10 {
        initial = make_insertions(initial, &insertions);
        dbg!(&initial.len());
    }

    let mut counts: HashMap<char, u32> = HashMap::new();
    for char in initial.chars() {
        *counts.entry(char).or_default() += 1;
    }
    // dbg!(&counts);

    let mut minc = u32::MAX;
    let mut maxc = 0;
    for ct in counts.values() {
        minc = min(minc, *ct);
        maxc = max(maxc, *ct);
    }
    maxc - minc
}

fn part_two(input: &str) -> u64 {
    let (_initial, mut insertions) = parse_input(input);

    for _ in 0..40 {
        insertions = make_insertions_faster(&insertions);
        /*print_insertions(&insertions);
        let tail = &insertions
            .values()
            .filter(|f| f.is_tail == true)
            .collect::<Vec<&Insertion>>();
        assert!(tail.len() == 1);
        dbg!(tail);
        dbg!(&insertions.values().map(|i| i.counter).sum::<u64>() + 1);*/
    }

    let mut counts: HashMap<char, u64> = HashMap::new();
    for insertion in insertions.values() {
        *counts.entry(insertion.pair[0]).or_default() += insertion.counter;
        if insertion.is_tail {
            *counts.entry(insertion.pair[1]).or_default() += 1; //insertion.counter;
        }
    }
    // dbg!(&counts);

    let mut minc = u64::MAX;
    let mut maxc = 0;
    for ct in counts.values() {
        minc = min(minc, *ct);
        maxc = max(maxc, *ct);
    }
    maxc - minc
}

fn main() {
    let input = include_str!("day14.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
