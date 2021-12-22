use std::cmp::{max, min};

// I know this should be Cube, sorry
#[derive(Debug, Clone)]
struct Rect {
    state: bool,
    x_start: i64,
    x_end: i64,
    y_start: i64,
    y_end: i64,
    z_start: i64,
    z_end: i64,
    subs: Vec<Box<Rect>>,
}

impl Rect {
    fn new(x1: i64, x2: i64, y1: i64, y2: i64, z1: i64, z2: i64) -> Rect {
        Rect {
            state: false,
            x_start: x1,
            x_end: x2,
            y_start: y1,
            y_end: y2,
            z_start: z1,
            z_end: z2,
            subs: Vec::new(),
        }
    }

    fn from_line(input: &str) -> Rect {
        // on x=10..12,y=10..12,z=10..12
        let parts: Vec<&str> = input.split(" ").collect();
        let bounds: Vec<&str> = parts[1].split(",").collect();
        let x: Vec<i64> = bounds[0].split("=").collect::<Vec<&str>>()[1]
            .split("..")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let y: Vec<i64> = bounds[1].split("=").collect::<Vec<&str>>()[1]
            .split("..")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let z: Vec<i64> = bounds[2].split("=").collect::<Vec<&str>>()[1]
            .split("..")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        Rect {
            state: parts[0] == "on",
            x_start: x[0],
            x_end: x[1],
            y_start: y[0],
            y_end: y[1],
            z_start: z[0],
            z_end: z[1],
            subs: Vec::new(),
        }
    }

    fn overlaps(&self, test: &Rect) -> bool {
        self.x_start <= test.x_end
            && self.y_start <= test.y_end
            && self.z_start <= test.z_end
            && self.x_end >= test.x_start
            && self.y_end >= test.y_start
            && self.z_end >= test.z_start
    }

    fn contains(&self, test: &Rect) -> bool {
        // True if self contains test entirely
        self.x_start <= test.x_start
            && self.x_end >= test.x_end
            && self.y_start <= test.y_start
            && self.y_end >= test.y_end
            && self.z_start <= test.z_start
            && self.z_end >= test.z_end
    }

    fn subtract(&self, test: &Rect) -> Vec<Rect> {
        // Return a list of rects that basically comprise the rect that is self
        // without test
        let mut rv = Vec::new();

        // Does not overlap: return self
        if !self.overlaps(test) {
            rv.push(self.clone());
            return rv;
        }

        // Test containts us entirely, so we no longer exist
        if test.contains(self) {
            return rv;
        }

        // dbg!(&self, &test);

        // We break ourselves into 6 pieces... top and bottom are full X/Z,
        // left/right are full Z, and front/back are smaller squares
        if self.y_start < test.y_start {
            // println!("top");
            rv.push(Rect::new(
                self.x_start,
                self.x_end,
                self.y_start,
                test.y_start - 1,
                self.z_start,
                self.z_end,
            ));
        }

        // Try to add the bottom
        if self.y_end > test.y_end {
            // println!("bot");
            rv.push(Rect::new(
                self.x_start,
                self.x_end,
                test.y_end + 1,
                self.y_end,
                self.z_start,
                self.z_end,
            ));
        }

        // We have now constrained the Y dimension for all further cubes
        // to be what the test cube has

        // Left
        if self.x_start < test.x_start {
            // println!("left");
            rv.push(Rect::new(
                self.x_start,
                test.x_start - 1,
                max(self.y_start, test.y_start),
                min(self.y_end, test.y_end),
                self.z_start,
                self.z_end,
            ));
        }

        // Right
        if self.x_end > test.x_end {
            // println!("right");

            rv.push(Rect::new(
                test.x_end + 1,
                self.x_end,
                max(self.y_start, test.y_start),
                min(self.y_end, test.y_end),
                self.z_start,
                self.z_end,
            ));
        }

        // We have now constrained the X dimension for all further cubes
        // to be what the test cube has

        // Front
        if self.z_start < test.z_start {
            // println!("ftont");

            rv.push(Rect::new(
                max(self.x_start, test.x_start),
                min(self.x_end, test.x_end),
                max(self.y_start, test.y_start),
                min(self.y_end, test.y_end),
                self.z_start,
                test.z_start - 1,
            ));
        }

        // Back
        if self.z_end > test.z_end {
            // println!("back");

            rv.push(Rect::new(
                max(self.x_start, test.x_start),
                min(self.x_end, test.x_end),
                max(self.y_start, test.y_start),
                min(self.y_end, test.y_end),
                test.z_end + 1,
                self.z_end,
            ));
        }

        rv
    }

    fn size(&self) -> i64 {
        (self.x_end - self.x_start + 1)
            * (self.y_end - self.y_start + 1)
            * (self.z_end - self.z_start + 1)
    }

    fn print(&self) {
        for x in self.x_start..=self.x_end {
            for y in self.y_start..=self.y_end {
                for z in self.z_start..=self.z_end {
                    println!("{},{},{}", x, y, z);
                }
            }
        }
    }
}

fn parse_input(input: &str, small_cube: bool) -> Vec<Rect> {
    let mut rv: Vec<Rect> = Vec::new();

    for line in input.split("\n") {
        let rect = Rect::from_line(line);

        // Only include cubes that overlap our starting area
        if !rect.overlaps(&Rect::new(-50, 50, -50, 50, -50, 50)) {
            if small_cube {
                continue;
            }
        }

        if rect.state {
            // This rect is on, which means we need to subtract everything we already
            // know about from it and continue doing so until everything left is just
            // the net new bits of 'on'
            let mut temp_rv = Vec::from_iter([rect]);

            for rect_to_subtract in &rv {
                let mut rects = Vec::new();

                for new_rect in temp_rv {
                    rects.append(&mut new_rect.subtract(rect_to_subtract));
                }

                temp_rv = rects;
            }

            rv.append(&mut temp_rv);
        } else {
            // Subtract this from any rects we know about
            let mut temp_rv = Vec::new();

            for existing_rect in &rv {
                temp_rv.append(&mut existing_rect.subtract(&rect));
            }

            // dbg!(&temp_rv);
            rv = temp_rv;
        }
    }

    rv
}

fn part_one(input: &str) -> i64 {
    let input = parse_input(input, true);

    let mut rv = 0;
    for rect in input {
        rv += rect.size();
    }
    rv
}

fn part_two(input: &str) -> i64 {
    let input = parse_input(input, false);

    let mut rv = 0;
    for rect in input {
        rv += rect.size();
    }
    rv
}

fn main() {
    let input = include_str!("day22.txt");
    println!("PART ONE: {}", part_one(input));
    println!("PART TWO: {}", part_two(input));
}

#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn it_works() {
        let rect = Rect::new(0, 10, 0, 10, 0, 10);

        assert!(rect.overlaps(&rect));
        assert!(rect.overlaps(&Rect::new(5, 8, 5, 8, 5, 8)));
        assert!(rect.overlaps(&Rect::new(5, 18, 5, 18, 5, 18)));
        assert!(rect.overlaps(&Rect::new(-5, 8, -5, 8, -5, 8)));
        assert!(!rect.overlaps(&Rect::new(15, 18, 5, 8, 5, 8)));
        assert!(rect.overlaps(&Rect::new(-5, 18, -5, 18, -5, 18)));
        assert!(!rect.overlaps(&Rect::new(-5, 8, -5, -1, 5, 8)));
    }
}
