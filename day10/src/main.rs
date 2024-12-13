use anyhow::Result;
use std::{fmt::Display, fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day10\\example.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

struct Tile {
    height: i32,
    x: usize,
    y: usize,
}

impl Tile {
    fn from_char(x: usize, y: usize, c: char) -> Self {
        match c {
            '0'..='9' => Tile {
                height: c.to_digit(10).unwrap() as i32,
                x,
                y,
            },
            c => panic!("Invalid character {}", c),
        }
    }
}

struct Island {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Island {
    fn from_string(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        let height = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            if width == 0 {
                width = line.chars().count();
            }

            for (x, c) in line.chars().enumerate() {
                let tile = Tile::from_char(x, y, c);
                tiles.push(tile);
            }
        }
        Island {
            width,
            height,
            tiles,
        }
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.tiles[y * self.width + x])
        }
    }

    fn find_trail_recursive(current_tile: &Tile, previous_tile: Option<&Tile>) -> usize {
        if current_tile.height == 9 {
            return 1;
        }
        if let Some(previous_tile) = previous_tile {
            if previous_tile.height < current_tile.height {
                return find_trail_recursive();
            }
        }
    }

    fn find_trail_from(x: usize, y: usize) -> usize {}
}

impl Display for Island {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = &self.tiles[y * self.width + x];
                write!(f, "{}", tile.height)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() {
    let data = retrieve_data().unwrap();
    let island = Island::from_string(&data);
    println!("{}", island);
}
