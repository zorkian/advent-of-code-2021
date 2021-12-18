use std::cmp::max;

#[derive(Debug, Clone)]
struct Pair {
    left_int: i32,
    left_pair: Option<Box<Pair>>,
    right_int: i32,
    right_pair: Option<Box<Pair>>,
}

impl Pair {
    fn new() -> Pair {
        Pair {
            left_int: -1,
            left_pair: None,
            right_int: -1,
            right_pair: None,
        }
    }

    fn is_empty(self: &Pair) -> bool {
        // True if this has not been initialized
        self.left_int == -1
            && self.right_int == -1
            && self.left_pair.is_none()
            && self.right_pair.is_none()
    }

    fn add(self: Pair, what: Pair) -> Pair {
        // Simple case, if we are yet empty, then the result is whatever we were given
        if self.is_empty() {
            return what;
        }

        // print!("Initial: ");
        // self.print();
        // println!("");

        // print!("Adding: ");
        // what.print();
        // println!("");

        // Create new tuple to store ourself in, push down
        let mut rv = Pair::new();
        rv.left_pair = Some(Box::new(self));
        rv.right_pair = Some(Box::new(what));

        // Simplify our return value
        loop {
            let (did_explode, _, _) = rv.reduce_explode(0);

            if !did_explode {
                let did_split = rv.reduce_split();

                if did_split {
                    // Back to the top, have to handle any explosions each time we have
                    // done a single split
                    // print!("Split: ");
                    // rv.print();
                    // println!("");
                    continue;
                }

                // No explosions, no splits, we're done
                break;
            } else {
                // print!("Exploded: ");
                // rv.print();
                // println!("");
            }
        }
        // print!("Final: ");
        // rv.print();
        // println!("");

        return rv;
    }

    fn left(self: &Pair) -> &Pair {
        return self.left_pair.as_ref().unwrap();
    }

    fn right(self: &Pair) -> &Pair {
        return self.right_pair.as_ref().unwrap();
    }

    fn left_mut(self: &mut Pair) -> &mut Pair {
        return self.left_pair.as_mut().unwrap();
    }

    fn right_mut(self: &mut Pair) -> &mut Pair {
        return self.right_pair.as_mut().unwrap();
    }

    fn add_leftmost(self: &mut Pair, value: i32) -> i32 {
        // Add a value to the leftmost integer that we can find, descending from
        // this point down... if we can't find one, return it up to be consumed
        // by somewhere else in the tree

        if self.left_pair.is_none() {
            self.left_int += value;
        } else {
            return self.left_mut().add_leftmost(value);
        }

        0
    }

    fn add_rightmost(self: &mut Pair, value: i32) -> i32 {
        // Add a value to the rightmost integer that we can find, descending from
        // this point down... if we can't find one, return it up to be consumed
        // by somewhere else in the tree

        if self.right_pair.is_none() {
            self.right_int += value;
        } else {
            return self.right_mut().add_rightmost(value);
        }

        0
    }

    fn magnitude(self: &Pair) -> i32 {
        let left_mag = if self.left_pair.is_some() {
            self.left().magnitude()
        } else {
            self.left_int
        };
        let right_mag = if self.right_pair.is_some() {
            self.right().magnitude()
        } else {
            self.right_int
        };
        (left_mag * 3) + (right_mag * 2)
    }

    fn reduce_explode(self: &mut Pair, depth: u32) -> (bool, i32, i32) {
        // [[[[[1,1],[2,2]],[3,3]],[4,4]],[5,5]]
        //      into
        // [[[[0,[3,2]],[3,3]],[4,4]],[5,5]]
        //      into
        // [[[[3,0],[5,3]],[4,4]],[5,5]]
        //
        // Start descending, once we hit depth 4, we know we have an explosion and
        // need to send up the value to get applied to the left/right if we were
        // unable to do so (values of -1 mean don't apply that side.)
        let (mut rv_left, mut rv_right) = (0, 0);

        // If we're deep enough to explode, do that if we need to by seeing if we have
        // any pairs down there, else leave them be
        if depth == 3 && (self.left_pair.is_some() || self.right_pair.is_some()) {
            // Any children get exploded and we will replace them with the 0 that is supposed to
            // be in that position
            let (mut keep_left, mut keep_right) = (0, 0);
            if self.left_pair.is_some() {
                // Assert we haven't fucked up
                assert!(self.left().left_pair.is_none());
                assert!(self.left().right_pair.is_none());

                // Get numbers we have to propogate out to the sides as well as to our own
                // right integer (we're depth 4, so there is guaranteed to be nothing but
                // integers below now)
                rv_left += self.left().left_int;
                keep_right += self.left().right_int;
                // dbg!(rv_left, keep_right);

                // Left side exploded
                self.left_int = 0;
                self.left_pair = None;

                // Since we've exploded something, we're done; start propogating up which will
                // stop the explosions here
                if self.right_pair.is_none() {
                    self.right_int += keep_right;
                } else {
                    rv_right += self.right_mut().add_leftmost(keep_right);
                }
                return (true, rv_left, rv_right);
            }

            if self.right_pair.is_some() {
                // Assert we haven't fucked up
                assert!(self.right().left_pair.is_none());
                assert!(self.right().right_pair.is_none());

                // Get numbers we have to propogate out to the sides
                keep_left += self.right().left_int;
                rv_right += self.right().right_int;
                // dbg!(keep_left, rv_right);

                // Zero out right side
                self.right_int = 0;
                self.right_pair = None;

                // Since we've exploded something, we're done; start propogating up which will
                // stop the explosions here
                if self.left_pair.is_none() {
                    self.left_int += keep_left;
                } else {
                    rv_left += self.left_mut().add_rightmost(keep_left);
                }
                return (true, rv_left, rv_right);
            }

            panic!("unreachable");
        }

        // We are not yet deep enough to explode, pass down the tree and ask if our
        // children need to explode ... if we get any numbers back, we need to try to apply
        // them to our children
        if self.left_pair.is_some() {
            let (did_explode, tll, keep_right) =
                self.left_pair.as_mut().unwrap().reduce_explode(depth + 1);

            if keep_right > 0 {
                if self.right_pair.is_some() {
                    rv_right += self.right_mut().add_leftmost(keep_right);
                } else {
                    self.right_int += keep_right;
                }
            }
            rv_left += tll;

            if did_explode {
                // Don't chain explode to the right
                return (true, rv_left, rv_right);
            }
        }
        if self.right_pair.is_some() {
            let (did_explode, keep_left, trr) =
                self.right_pair.as_mut().unwrap().reduce_explode(depth + 1);

            if keep_left > 0 {
                if self.left_pair.is_some() {
                    rv_left += self.left_mut().add_rightmost(keep_left);
                } else {
                    self.left_int += keep_left;
                }
            }
            rv_right += trr;

            if did_explode {
                // Don't chain explode to the right
                return (true, rv_left, rv_right);
            }
        }

        // If we have some values to keep, try to keep those into the pair if we have it, else
        // keep it into our own int

        // Nothing doing, boss
        assert_eq!(rv_left, 0);
        assert_eq!(rv_right, 0);
        return (false, rv_left, rv_right);
    }

    fn reduce_split(self: &mut Pair) -> bool {
        // See if we have an integer that is too big and if so, split according to
        // the simple rules and return
        if self.left_pair.is_some() {
            if self.left_mut().reduce_split() {
                return true;
            }
        } else {
            if self.left_int >= 10 {
                self.left_pair = Some(Box::new(Pair::new()));
                self.left_mut().left_int = self.left_int / 2;
                self.left_mut().right_int = (self.left_int / 2) + (self.left_int % 2);
                self.left_int = -1;
                return true;
            }
        }

        // Try split right
        if self.right_pair.is_some() {
            if self.right_mut().reduce_split() {
                return true;
            }
        } else {
            if self.right_int >= 10 {
                self.right_pair = Some(Box::new(Pair::new()));
                self.right_mut().left_int = self.right_int / 2;
                self.right_mut().right_int = (self.right_int / 2) + (self.right_int % 2);
                self.right_int = -1;
                return true;
            }
        }

        false
    }

    fn print(self: &Pair) {
        print!("[");
        if self.left_int > -1 {
            print!("{}", self.left_int);
        } else {
            self.left().print();
        }
        print!(",");
        if self.right_int > -1 {
            print!("{}", self.right_int);
        } else {
            self.right().print();
        }
        print!("]");
    }
}

fn parse_pair(input: &[u8], pos: usize) -> (Pair, usize) {
    // pos will be the index of the first character to start parsing
    // return usize is the index that we just finished parsing
    let mut rv = Pair::new();
    let mut is_left = true;
    let mut idx = pos;
    loop {
        if idx >= input.len() {
            break;
        }
        match input[idx] {
            b'[' => {
                // We're starting a new pair, so parse it and stick it in the proper place
                let (subpair, new_idx) = parse_pair(input, idx + 1);
                idx = new_idx;
                if is_left {
                    rv.left_pair = Some(Box::new(subpair));
                } else {
                    rv.right_pair = Some(Box::new(subpair));
                }
            }
            b',' => is_left = false,
            b']' => return (rv, idx),
            b'0'..=b'9' => {
                let int = input[idx] - b'0';
                if is_left {
                    rv.left_int = int as i32;
                } else {
                    rv.right_int = int as i32;
                }
            }
            _ => panic!("Unknown character: {}", input[idx]),
        }
        idx += 1;
    }
    panic!("got too far");
}

fn part_one(input: &str) -> u32 {
    let mut root = Pair::new();

    for line in input.split("\n") {
        let (pair, _) = parse_pair(line.as_bytes(), 1);
        root = root.add(pair);
    }

    root.magnitude() as u32
}

fn part_two(input: &str) -> u32 {
    let pairs: Vec<&[u8]> = input.split("\n").map(|l| l.as_bytes()).collect();
    let mut rv = 0;

    for test_idx in 0..pairs.len() {
        for against_idx in 0..pairs.len() {
            if test_idx == against_idx {
                continue;
            }

            let (mut test_pair, _) = parse_pair(pairs[test_idx], 1);
            let (against_pair, _) = parse_pair(pairs[against_idx], 1);
            test_pair = test_pair.add(against_pair);
            rv = max(rv, test_pair.magnitude());
        }
    }

    rv as u32
}

fn main() {
    let input = include_str!("day18.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
