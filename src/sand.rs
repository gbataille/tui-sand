use crossterm::{cursor, ExecutableCommand};
use std::collections::HashSet;
use std::error::Error;
use std::io::Write;
use std::vec::Vec;

pub struct World {
    rocks_coord: HashSet<(i32, i32)>,
    moving_sand_coord: (i32, i32),
    sands_coord: HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl World {
    fn new() -> Self {
        World {
            rocks_coord: HashSet::new(),
            moving_sand_coord: (500, 0),
            sands_coord: HashSet::new(),
            min_x: std::i32::MAX,
            max_x: 0,
            min_y: 0,
            max_y: 0,
        }
    }

    fn add_rock(&mut self, coord: (i32, i32)) {
        self.rocks_coord.insert(coord);
        if coord.0 < self.min_x {
            self.min_x = coord.0;
        }
        if coord.0 > self.max_x {
            self.max_x = coord.0;
        }
        if coord.1 < self.min_y {
            self.min_y = coord.1;
        }
        if coord.1 > self.max_y {
            self.max_y = coord.1;
        }
    }

    fn add_sand(&mut self, coord: (i32, i32)) {
        self.sands_coord.insert(coord);
        if coord.0 < self.min_x {
            self.min_x = coord.0;
        }
        if coord.0 > self.max_x {
            self.max_x = coord.0;
        }
        if coord.1 < self.min_y {
            self.min_y = coord.1;
        }
        if coord.1 > self.max_y {
            self.max_y = coord.1;
        }
    }

    fn add_floor(&mut self) {
        for x in self.min_x..=self.max_x {
            self.rocks_coord.insert((x, self.max_y + 2));
        }
        self.max_y += 2;
    }

    fn is_occupied(&self, coord: (i32, i32)) -> bool {
        self.rocks_coord.get(&coord).is_some() || self.sands_coord.get(&coord).is_some()
    }

    fn is_occupied_part2(&self, coord: (i32, i32)) -> bool {
        self.rocks_coord.get(&coord).is_some()
            || self.sands_coord.get(&coord).is_some()
            || coord.1 == self.max_y
    }

    fn is_out(&self, coord: (i32, i32)) -> bool {
        coord.0 < self.min_x || coord.0 > self.max_x || coord.1 > self.max_y || coord.1 < self.min_y
    }

    pub fn move_step_part2(&mut self) -> Result<(), Box<dyn Error>> {
        for attempt in [
            (self.moving_sand_coord.0, self.moving_sand_coord.1 + 1),
            (self.moving_sand_coord.0 - 1, self.moving_sand_coord.1 + 1),
            (self.moving_sand_coord.0 + 1, self.moving_sand_coord.1 + 1),
        ] {
            if !self.is_occupied_part2(attempt) {
                self.moving_sand_coord = attempt;
                if attempt.0 < self.min_x {
                    self.min_x = attempt.0;
                } else if attempt.0 > self.max_x {
                    self.max_x = attempt.0;
                }
                if attempt.1 < self.min_y {
                    self.min_y = attempt.1;
                } else if attempt.1 > self.max_y {
                    self.max_y = attempt.1;
                }
                return Ok(());
            }
        }

        // did not move, so settled

        // did it clog?
        if self.moving_sand_coord == (500, 0) {
            self.add_sand(self.moving_sand_coord);
            return Err("clogged")?;
        }

        self.add_sand(self.moving_sand_coord);
        self.moving_sand_coord = (500, 0);

        Ok(())
    }

    pub fn move_step(&mut self) -> Result<(), Box<dyn Error>> {
        for attempt in [
            (self.moving_sand_coord.0, self.moving_sand_coord.1 + 1),
            (self.moving_sand_coord.0 - 1, self.moving_sand_coord.1 + 1),
            (self.moving_sand_coord.0 + 1, self.moving_sand_coord.1 + 1),
        ] {
            if self.is_out(attempt) {
                return Err(format!("Out at {:?}", attempt))?;
            }
            if !self.is_occupied(attempt) {
                self.moving_sand_coord = attempt;
                return Ok(());
            }
        }

        // did not move, so settled
        self.sands_coord.insert(self.moving_sand_coord);
        self.moving_sand_coord = (500, 0);

        Ok(())
    }

    pub fn display_to_term(&self, out: &mut std::io::Stdout) -> Result<(), std::io::Error> {
        for y in self.min_y..=self.max_y {
            for x in self.min_x - 2..=self.max_x + 2 {
                out.execute(cursor::MoveTo(x as u16 - 300, y as u16));
                if y == 0 && x == 500 {
                    if self.sands_coord.get(&(x, y)).is_some() {
                        out.write(b"X")?;
                    } else {
                        out.write(b"+")?;
                    }
                } else if x == self.moving_sand_coord.0 && y == self.moving_sand_coord.1 {
                    out.write(b"o")?;
                } else if self.sands_coord.get(&(x, y)).is_some() {
                    out.write(b"O")?;
                } else if self.rocks_coord.get(&(x, y)).is_some() {
                    out.write(b"#")?;
                } else if y == self.max_y {
                    out.write(b"~")?;
                } else {
                    out.write(b".")?;
                }
            }
            out.write(b"")?;
        }
        out.flush()
    }
}

fn part1(contents: &String) {
    let mut world = parse_input(contents);

    // world.display();

    let mut step = 1;
    loop {
        println!("Step {}", step);

        match world.move_step() {
            Ok(_) => (),
            Err(a) => {
                println!("End with: {}", a);
                break;
            }
        }

        // world.display();

        step += 1;
    }

    // world.display();
    println!("\nNb settled sand {}", world.sands_coord.len());
}

fn part2(contents: &String) {
    let mut world = World::new();
    for line in contents.lines() {
        parse_line(line, &mut world);
    }
    world.add_floor();

    // world.display();

    let mut step = 1;
    loop {
        println!("Step {}", step);

        match world.move_step_part2() {
            Ok(_) => (),
            Err(a) => {
                println!("End with: {}", a);
                break;
            }
        }

        // world.display();

        step += 1;
    }

    // world.display();
    println!("\nNb settled sand {}", world.sands_coord.len());
}

pub fn parse_input(input: &str) -> World {
    let mut world = World::new();
    for line in input.lines() {
        parse_line(line, &mut world);
    }
    world
}

fn parse_line(line: &str, world: &mut World) {
    let mut corners: Vec<(i32, i32)> = Vec::new();

    for corner in line.split(" -> ") {
        let coords = corner.split(",").collect::<Vec<&str>>();
        let (x_str, y_str) = (coords[0], coords[1]);
        let x = x_str.parse::<i32>().unwrap();
        let y = y_str.parse::<i32>().unwrap();
        corners.push((x, y));
    }

    for i_corner in 1..corners.len() {
        let a = corners[i_corner - 1];
        let b = corners[i_corner];

        let from_x = a.0.min(b.0);
        let to_x = a.0.max(b.0);
        let from_y = a.1.min(b.1);
        let to_y = a.1.max(b.1);

        for i_x in from_x..=to_x {
            for i_y in from_y..=to_y {
                world.add_rock((i_x, i_y));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parts() {
        let test_contents = <String as std::str::FromStr>::from_str(
            "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
        )
        .unwrap();

        part1(&test_contents);
        part2(&test_contents);
    }
}
