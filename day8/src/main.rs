use anyhow::Result;
use std::{fmt::Display, fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day8\\day8.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Clone, PartialEq, Debug)]
enum CellType {
    Blank,
    Antenna,
}

#[derive(Clone, Debug)]
struct Cell {
    cell_type: CellType,
    x: usize,
    y: usize,
    identifier: Option<char>,
    is_antinode: bool,
}

impl Cell {
    fn from_character(character: char, x: usize, y: usize) -> Cell {
        match character {
            '.' => Cell {
                cell_type: CellType::Blank,
                x,
                y,
                identifier: None,
                is_antinode: false,
            },
            c if c.is_alphanumeric() => Cell {
                cell_type: CellType::Antenna,
                x,
                y,
                identifier: Some(c),
                is_antinode: false,
            },
            _ => panic!("Invalid character"),
        }
    }

    fn diff(&self, other: &Cell) -> (i32, i32) {
        let x_diff = other.x as i32 - self.x as i32;
        let y_diff = other.y as i32 - self.y as i32;
        (x_diff, y_diff)
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.cell_type {
            CellType::Blank if self.is_antinode => write!(f, "#"),
            CellType::Blank => write!(f, "."),
            CellType::Antenna => write!(f, "{}", self.identifier.unwrap().to_string()),
        }
    }
}

struct Map {
    vector: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_string(data: &str) -> Map {
        let mut vector = Vec::new();
        let mut width = 0;
        for (y, line) in data.lines().enumerate() {
            if width == 0 {
                width = line.len();
            }
            for (x, character) in line.chars().enumerate() {
                let cell = Cell::from_character(character, x, y);
                vector.push(cell);
            }
        }
        Map {
            vector,
            width,
            height: data.lines().count(),
        }
    }

    fn get_antennas(&self) -> Vec<Cell> {
        self.vector
            .iter()
            .filter(|c| c.cell_type == CellType::Antenna)
            .cloned()
            .collect()
    }

    fn set_antinode(&mut self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return false;
        }

        let index = self
            .vector
            .iter()
            .position(|c| c.x == x as usize && c.y == y as usize)
            .unwrap();
        self.vector[index].is_antinode = true;
        true
    }

    fn set_antinode_recursive(&mut self, x: i32, y: i32, diff_x: i32, diff_y: i32) {
        if !self.set_antinode(x, y) {
            return;
        }

        self.set_antinode_recursive(x + diff_x, y + diff_y, diff_x, diff_y);
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        let mut prev_y = 0;
        for cell in &self.vector {
            if cell.y != prev_y {
                result.push_str("\n");
                prev_y = cell.y;
            }
            result.push_str(&cell.to_string());
        }
        write!(f, "{}", result)
    }
}

fn main() {
    let data = retrieve_data().unwrap();
    let mut m = Map::from_string(&data);

    let antennas = m.get_antennas();
    println!("{:?}", antennas);
    let mut identifiers = Vec::new();

    for antenna in &antennas {
        let ident = antenna.identifier.unwrap();
        if !identifiers.contains(&ident) {
            identifiers.push(ident);
        }
    }

    println!("{}", m);

    for identifier in &identifiers {
        let antennas = antennas
            .iter()
            .filter(|a| a.identifier.unwrap() == *identifier)
            .cloned()
            .collect::<Vec<Cell>>();

        for (idx, this_antenna) in antennas.iter().enumerate() {
            let mut antennas_without = antennas.clone();
            antennas_without.remove(idx);

            for other_antenna in antennas_without {
                let (diff_x, diff_y) = this_antenna.diff(&other_antenna);

                m.set_antinode_recursive(
                    this_antenna.x as i32,
                    this_antenna.y as i32,
                    diff_x,
                    diff_y,
                );

                m.set_antinode_recursive(
                    this_antenna.x as i32,
                    this_antenna.y as i32,
                    diff_x * -1,
                    diff_y * -1,
                );
            }
        }
    }

    println!("{}", m);

    println!(
        "total antinodes: {}",
        m.vector.iter().filter(|c| c.is_antinode).count(),
    );
}
