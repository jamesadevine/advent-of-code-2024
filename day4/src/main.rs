use anyhow::{anyhow, Result};
use std::fmt::Display;
use std::{fs::File, io::Read};

static XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
static MAS: [char; 3] = ['M', 'A', 'S'];

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day4\\day4.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<String>>,
}

impl Grid {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
    fn from_text(text: &str) -> Grid {
        let mut grid = Vec::new();
        for line in text.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_string());
            }
            grid.push(row);
        }
        Grid {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&String> {
        if y >= self.height as i32 || y < 0 || x < 0 || x >= self.width as i32 {
            return None;
        }
        Some(&self.grid[y as usize][x as usize])
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for c in row {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn analyze(
    match_index: usize,
    match_seq: &[char],
    grid: &Grid,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
) -> usize {
    if match_index == match_seq.len() {
        return 1;
    }

    match grid.get(x, y) {
        Some(c) => {
            if c == &match_seq[match_index].to_string() {
                analyze(match_index + 1, match_seq, grid, x + dx, y + dy, dx, dy)
            } else {
                0
            }
        }
        None => 0,
    }
}

fn count_xmas(grid: &Grid) -> usize {
    let mut xmas_count = 0;
    // find all XMAS
    for y in 0..grid.width() {
        for x in 0..grid.height() {
            // set a search in each direction
            let x = x as i32;
            let y = y as i32;
            xmas_count += analyze(0, &XMAS, &grid, x, y, 1, 0)
                + analyze(0, &XMAS, &grid, x, y, 0, 1)
                + analyze(0, &XMAS, &grid, x, y, 1, 1)
                + analyze(0, &XMAS, &grid, x, y, -1, -1)
                + analyze(0, &XMAS, &grid, x, y, -1, 0)
                + analyze(0, &XMAS, &grid, x, y, 0, -1)
                + analyze(0, &XMAS, &grid, x, y, 1, -1)
                + analyze(0, &XMAS, &grid, x, y, -1, 1);
        }
    }
    xmas_count
}

#[derive(PartialEq)]
enum MasDirection {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl TryFrom<usize> for MasDirection {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MasDirection::UpperLeft),
            1 => Ok(MasDirection::UpperRight),
            2 => Ok(MasDirection::LowerLeft),
            3 => Ok(MasDirection::LowerRight),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}

#[derive(PartialEq)]
struct Mas {
    x: i32,
    y: i32,
    direction: MasDirection,
}

impl Mas {
    fn new(x: i32, y: i32, direction: MasDirection) -> Mas {
        Mas { x, y, direction }
    }

    fn with_direction(x: i32, y: i32, direction: usize) -> Result<Mas> {
        Ok(Mas {
            x,
            y,
            direction: MasDirection::try_from(direction)?,
        })
    }

    fn intersects(&self, other: &Mas) -> bool {
        let m1_middle_coords = match self.direction {
            MasDirection::UpperLeft => (self.x - 1, self.y + 1),
            MasDirection::UpperRight => (self.x + 1, self.y + 1),
            MasDirection::LowerLeft => (self.x - 1, self.y - 1),
            MasDirection::LowerRight => (self.x + 1, self.y - 1),
        };

        let m2_middle_coords = match other.direction {
            MasDirection::UpperLeft => (other.x - 1, other.y + 1),
            MasDirection::UpperRight => (other.x + 1, other.y + 1),
            MasDirection::LowerLeft => (other.x - 1, other.y - 1),
            MasDirection::LowerRight => (other.x + 1, other.y - 1),
        };

        m1_middle_coords == m2_middle_coords
    }
}

fn find_mas(grid: &Grid) -> Vec<Mas> {
    let mut mas = Vec::new();
    // find all MAS
    for y in 0..grid.width() {
        for x in 0..grid.height() {
            // set a search in all diagonal directions
            let x = x as i32;
            let y = y as i32;
            let matches = vec![
                analyze(0, &MAS, &grid, x, y, -1, 1),  // upper left
                analyze(0, &MAS, &grid, x, y, 1, 1),   // upper right
                analyze(0, &MAS, &grid, x, y, -1, -1), // lowerleft
                analyze(0, &MAS, &grid, x, y, 1, -1),  // lower right
            ];

            for (idx, m) in matches.iter().enumerate() {
                if *m > 0 {
                    match Mas::with_direction(x, y, idx) {
                        Ok(mas_) => mas.push(mas_),
                        Err(_) => (),
                    }
                }
            }
        }
    }
    mas
}

fn count_mas(grid: &Grid) -> usize {
    let mut mas_count = 0;
    let mas = find_mas(&grid);

    println!("mas count: {}", mas.len());
    for this_m in &mas {
        for other_m in &mas {
            if this_m == other_m {
                continue;
            }
            // if considered_mas.contains(&other_m) {
            //     continue;
            // }
            if this_m.intersects(&other_m) {
                mas_count += 1;
            }
        }
    }
    // account for the double counting valid MAS!
    mas_count / 2
}

fn main() {
    let grid_src = retrieve_data().unwrap();
    let grid = Grid::from_text(&grid_src);

    let xmas_count = count_xmas(&grid);
    let mas_count = count_mas(&grid);
    println!("{}", grid);
    println!("Total XMAS count: {}", xmas_count);
    println!("Total X-MAS count: {}", mas_count);
}
