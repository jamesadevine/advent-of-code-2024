use anyhow::Result;
use std::{clone, fmt::Display, fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day6\\day6.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Clone, PartialEq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Clone, Debug)]
enum SquareType {
    Standard,
    Obstacle,
    Guard,
}

#[derive(Clone)]
struct Square {
    x: i32,
    y: i32,
    square_type: SquareType,
    direction: Direction,
    last_visit: Option<Direction>,
    new_obstacle: bool,
}

impl Square {
    fn visit(&mut self, direction: &Direction) -> MapState {
        // println!(
        //     "Visiting square at {}, {} with square_type {:?}",
        //     self.x, self.y, self.square_type
        // );
        match self.square_type {
            SquareType::Standard => match &self.last_visit {
                Some(last_direction) => {
                    if *last_direction == direction.clone() {
                        return MapState::Loop;
                    }
                }
                None => self.last_visit = Some(direction.clone()),
            },
            SquareType::Obstacle => panic!("Obstacle cannot be visited"),
            SquareType::Guard => panic!("Guard cannot be visited"),
        }
        MapState::Continue
    }
    fn from_char(x: usize, y: usize, c: char) -> Self {
        let x = x as i32;
        let y = y as i32;
        match c {
            '.' => Self {
                x,
                y,
                square_type: SquareType::Standard,
                direction: Direction::None,
                last_visit: None,
                new_obstacle: false,
            },
            '#' => Self {
                x,
                y,
                square_type: SquareType::Obstacle,
                direction: Direction::None,
                last_visit: None,
                new_obstacle: false,
            },
            '^' => Self {
                x,
                y,
                square_type: SquareType::Guard,
                direction: Direction::Up,
                last_visit: None,
                new_obstacle: false,
            },
            'O' => Self {
                x,
                y,
                square_type: SquareType::Obstacle,
                direction: Direction::None,
                last_visit: None,
                new_obstacle: true,
            },
            _ => panic!("Invalid character {}", c),
        }
    }

    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
            Direction::Right => self.direction = Direction::Down,
            Direction::None => panic!("Invalid direction"),
        }
    }

    fn update(&mut self, map_width: usize, map_height: usize) -> bool {
        match self.square_type {
            SquareType::Standard => {}
            SquareType::Obstacle => {}
            SquareType::Guard => match self.direction {
                Direction::Up => {
                    self.y -= 1;
                }
                Direction::Down => {
                    self.y += 1;
                }
                Direction::Left => {
                    self.x -= 1;
                }
                Direction::Right => {
                    self.x += 1;
                }
                Direction::None => panic!("Invalid direction"),
            },
        }

        self.x < 0 || self.y < 0 || self.x >= map_width as i32 || self.y >= map_height as i32
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.square_type {
            SquareType::Standard => {
                if self.last_visit.is_some() {
                    write!(f, "x")
                } else {
                    write!(f, ".")
                }
            }
            SquareType::Obstacle => {
                if self.new_obstacle {
                    write!(f, "O")
                } else {
                    write!(f, "#")
                }
            }
            SquareType::Guard => match self.direction {
                Direction::Up => write!(f, "^"),
                Direction::Down => write!(f, "v"),
                Direction::Left => write!(f, "<"),
                Direction::Right => write!(f, ">"),
                Direction::None => panic!("Invalid direction"),
            },
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<Square>>,
    route: Vec<(i32, i32, Direction)>,
    guard_index: (usize, usize),
}

enum MapState {
    Loop,
    OutOfBounds,
    Continue,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (guard_x, guard_y) = self
            .map
            .iter()
            .flatten()
            .find(|sq| sq.square_type == SquareType::Guard)
            .map(|sq| (sq.x, sq.y))
            .unwrap();

        for row in &self.map {
            for cell in row {
                if cell.x == guard_x
                    && cell.y == guard_y
                    && cell.square_type == SquareType::Standard
                {
                    continue;
                }
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn move_guard(current_guard_x: i32, current_guard_y: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (current_guard_x, current_guard_y - 1),
        Direction::Down => (current_guard_x, current_guard_y + 1),
        Direction::Left => (current_guard_x - 1, current_guard_y),
        Direction::Right => (current_guard_x + 1, current_guard_y),
        Direction::None => panic!("Invalid direction"),
    }
}

impl Map {
    fn add_stop(&mut self, x: i32, y: i32, direction: Direction) -> bool {
        match self
            .route
            .iter()
            .find(|(x1, y1, dir)| x == *x1 && y == *y1 && *dir == direction)
        {
            Some(_) => {
                println!("Loop detected at {}, {}", x, y);
                self.route.push((x, y, direction));
                true
            }
            None => {
                self.route.push((x, y, direction));
                false
            }
        }
    }
    fn from_initial_state(parsed: &Vec<Vec<char>>) -> Self {
        let mut map = Vec::new();
        let mut guard_x = 0;
        let mut guard_y = 0;

        for (y, row) in parsed.iter().enumerate() {
            let mut square_row = Vec::new();

            for (x, &c) in row.iter().enumerate() {
                let sq = Square::from_char(x, y, c);

                if sq.square_type == SquareType::Guard {
                    guard_x = x;
                    guard_y = y;
                }
                square_row.push(sq);
            }

            map.push(square_row);
        }

        let width = map[0].len();
        let height = map.len();

        Self {
            width,
            height,
            map,
            route: vec![],
            guard_index: (guard_x, guard_y),
        }
    }

    fn visited(&self) -> usize {
        self.map
            .iter()
            .flatten()
            .filter(|sq| sq.last_visit.is_some())
            .count()
            + 1
    }

    fn update(&mut self) -> MapState {
        let (current_guard_x, current_guard_y) = self.guard_index;

        let guard_x = self.map[current_guard_y][current_guard_x].x;
        let guard_y = self.map[current_guard_y][current_guard_x].y;
        let guard_direction = &self.map[current_guard_y][current_guard_x].direction.clone();

        let (new_guard_x, new_guard_y) = move_guard(guard_x, guard_y, guard_direction);

        // complete condition, out of bounds
        if new_guard_x < 0
            || new_guard_y < 0
            || new_guard_x >= self.width as i32
            || new_guard_y >= self.height as i32
        {
            return MapState::OutOfBounds;
        }

        let (new_guard_x, new_guard_y) = (new_guard_x as usize, new_guard_y as usize);

        if self.map[new_guard_y][new_guard_x].square_type == SquareType::Obstacle {
            self.map[current_guard_y][current_guard_x].rotate();
            return MapState::Continue;
        }

        self.map[current_guard_y][current_guard_x].square_type = SquareType::Standard;
        match self.map[current_guard_y][current_guard_x].visit(&guard_direction) {
            MapState::Loop => return MapState::Loop,
            MapState::OutOfBounds => return MapState::OutOfBounds,
            MapState::Continue => {
                self.map[new_guard_y][new_guard_x].square_type = SquareType::Guard;
                self.map[new_guard_y][new_guard_x].direction = guard_direction.clone();

                self.guard_index = (new_guard_x, new_guard_y);
            }
        }
        MapState::Continue
    }
}

fn main() {
    let data = retrieve_data().unwrap();

    let parsed: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut vec_seeds = vec![];
    for (y, row) in parsed.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == '.' {
                let mut new_seed = parsed.clone();
                new_seed[y][x] = 'O';
                vec_seeds.push(new_seed);
            }
        }
    }

    let mut looping_seed_count = 0;
    for (idx, seed) in vec_seeds.iter().enumerate() {
        println!("Seed: {}/{}", idx, &vec_seeds.len());
        let mut map = Map::from_initial_state(&seed);
        // println!("{}", map);
        loop {
            match map.update() {
                MapState::Loop => {
                    looping_seed_count += 1;
                    break;
                }
                MapState::OutOfBounds => break,
                MapState::Continue => {}
            }
        }
    }

    println!(
        "Looping seed count: {}/{}",
        looping_seed_count,
        &vec_seeds.len()
    );
}
