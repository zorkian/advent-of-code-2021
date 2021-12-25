use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Opcode {
    Unknown,
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

#[derive(Debug)]
struct Instruction {
    op: Opcode,
    left_var: char,
    right_val: Option<i64>,
    right_var: Option<char>,
}

impl Instruction {
    fn new() -> Instruction {
        Instruction {
            op: Opcode::Unknown,
            left_var: ' ',
            right_val: None,
            right_var: None,
        }
    }

    fn value(&self, vars: &HashMap<char, i64>) -> i64 {
        if let Some(right_var) = self.right_var {
            return *vars.get(&right_var).unwrap_or(&0);
        }
        if let Some(right_val) = self.right_val {
            return right_val;
        }
        panic!("instruction with no var/val");
    }

    fn execute(&self, input: i64, vars: &mut HashMap<char, i64>) {
        let value = match self.op {
            Opcode::Unknown => panic!("unknown"),
            Opcode::Input => input,
            Opcode::Add => *vars.get(&self.left_var).unwrap_or(&0) + self.value(&vars),
            Opcode::Multiply => *vars.get(&self.left_var).unwrap_or(&0) * self.value(&vars),
            Opcode::Divide => *vars.get(&self.left_var).unwrap_or(&0) / self.value(&vars),
            Opcode::Modulo => *vars.get(&self.left_var).unwrap_or(&0) % self.value(&vars),
            Opcode::Equal => {
                if *vars.get(&self.left_var).unwrap_or(&0) == self.value(&vars) {
                    1
                } else {
                    0
                }
            }
        };

        // Now assign to output
        // dbg!(self.left_var, value);
        *vars.entry(self.left_var).or_insert(0) = value;
    }
}

fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    let mut rv = Vec::new();

    let mut temp_rv = Vec::new();
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        let mut inst = Instruction::new();
        inst.op = match parts[0] {
            "inp" => Opcode::Input,
            "add" => Opcode::Add,
            "mul" => Opcode::Multiply,
            "div" => Opcode::Divide,
            "mod" => Opcode::Modulo,
            "eql" => Opcode::Equal,
            _ => Opcode::Unknown,
        };

        if inst.op == Opcode::Unknown {
            continue;
        }

        if parts.len() >= 2 {
            inst.left_var = parts[1].chars().next().unwrap();
        }
        if parts.len() >= 3 {
            let test_char = parts[2].chars().next().unwrap();
            match test_char {
                'a'..='z' => inst.right_var = Some(test_char),
                _ => inst.right_val = Some(parts[2].parse::<i64>().unwrap()),
            }
        }

        // If this is an input, start a new character section
        if inst.op == Opcode::Input {
            if temp_rv.len() > 0 {
                rv.push(temp_rv);
                temp_rv = Vec::new();
            }
        }

        temp_rv.push(inst);
    }

    if temp_rv.len() > 0 {
        rv.push(temp_rv);
    }

    rv
}

fn part_one(input: &str) -> u32 {
    let mut input = parse_input(input);

    // section, digit, w, x, y, z -> w, x, y, z
    let mut cache: HashMap<(usize, i64, i64, i64, i64, i64), (i64, i64, i64, i64)> = HashMap::new();

    let mut digits = [9i64, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 0];
    loop {
        let mut vars: HashMap<char, i64> = HashMap::new();
        let mut digit_idx: usize = 0;

        // println!("Test {:?}", digits);
        for idx in 0..input.len() {
            // If this one is cached, reset registers and continue
            let cache_key = (
                idx,
                digits[digit_idx],
                *vars.get(&'w').unwrap_or(&0),
                *vars.get(&'x').unwrap_or(&0),
                *vars.get(&'y').unwrap_or(&0),
                *vars.get(&'z').unwrap_or(&0),
            );
            let cached = cache.get(&cache_key);
            if let Some(cached_vars) = cached {
                // println!("Hit cache with {:?}", cache_key);
                *vars.entry('w').or_insert(0) = cached_vars.0;
                *vars.entry('x').or_insert(0) = cached_vars.1;
                *vars.entry('y').or_insert(0) = cached_vars.2;
                *vars.entry('z').or_insert(0) = cached_vars.3;
                digit_idx += 1;
                continue;
            }

            // Execute it
            for inst in &input[idx] {
                inst.execute(digits[digit_idx], &mut vars);
                if inst.op == Opcode::Input {
                    digit_idx += 1;
                }
            }

            // Now cache the result
            cache.entry(cache_key).or_insert((
                *vars.get(&'w').unwrap_or(&0),
                *vars.get(&'x').unwrap_or(&0),
                *vars.get(&'y').unwrap_or(&0),
                *vars.get(&'z').unwrap_or(&0),
            ));
        }

        if let Some(val) = vars.get(&'z') {
            if *val == 0 {
                println!("{:?} is valid", digits);
            }
        }

        // Decrement
        for incr_idx in (0..14).rev() {
            if incr_idx == 9 {
                println!("Now at {:?}", digits);
            }
            digits[incr_idx] -= 1;
            if digits[incr_idx] == 0 {
                digits[incr_idx] = 9;
            } else {
                break;
            }
        }
    }

    0
}

fn part_two(input: &str) -> u32 {
    let mut input = parse_input(input);

    0
}

fn main() {
    let input = include_str!("day24.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}
