use anyhow::Result;
use std::{fs::File, io::Read};

enum Orientation {
    Increasing,
    Decreasing,
}

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day2\\day2.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_data(content: &str) -> Result<Vec<Vec<i32>>> {
    let mut data = Vec::new();
    for line in content.lines() {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse::<i32>()?);
        }
        data.push(row);
    }
    Ok(data)
}

fn detect_orientation(row: &Vec<i32>) -> Orientation {
    let mut greater_count = 0;
    let mut lesser_count = 0;

    let root = row[0];

    for i in 1..row.len() - 1 {
        // bias towards counting up
        if row[i] >= root {
            greater_count += 1;
        } else if row[i] < root {
            lesser_count += 1;
        }
    }

    if greater_count > lesser_count {
        Orientation::Increasing
    } else {
        Orientation::Decreasing
    }
}

// fn detect_orientation(row: &Vec<i32>) -> Orientation {
//     let total = row.iter().sum::<i32>();
//     let average = total / (row.len() as i32);
//     if average > row[0] {
//         Orientation::Increasing
//     } else {
//         Orientation::Decreasing
//     }
// }

fn all_increasing(row: &Vec<i32>) -> Option<i32> {
    for i in 0..row.len() - 1 {
        // if current is less than next
        if row[i] < row[i + 1] {
            // if the difference is greater than 3
            let diff = row[i + 1] - row[i];
            if diff > 3 && diff > 0 {
                return Some((i + 1) as i32);
            }
        } else {
            return Some(i as i32);
        }
    }
    None
}

fn all_decreasing(row: &Vec<i32>) -> Option<i32> {
    for i in 0..row.len() - 1 {
        if row[i] > row[i + 1] {
            let diff = row[i] - row[i + 1];
            if diff > 3 && diff > 0 {
                return Some((i + 1) as i32);
            }
        } else {
            return Some(i as i32);
        }
    }
    None
}

fn check_row(row: &Vec<i32>) -> Option<i32> {
    match (all_increasing(row), all_decreasing(row)) {
        (Some(i), Some(j)) => match detect_orientation(row) {
            Orientation::Increasing => Some(i),
            Orientation::Decreasing => Some(j),
        },
        _ => None,
    }
}

fn summarise(data: &Vec<Vec<i32>>, dampening: bool) -> (i32, i32) {
    let mut safe_count = 0;
    let mut unsafe_count = 0;

    for row in data {
        if !dampening {
            if check_row(row).is_none() {
                safe_count += 1;
            } else {
                unsafe_count += 1;
            }
        } else {
            let violations = check_row(row);
            if violations.is_none() {
                safe_count += 1;
            } else {
                println!("Row: {:?} is unsafe. Checking for dampening criteria", row);
                let index = violations.unwrap();
                let mut now_safe = false;
                for i in 0..row.len() {
                    let mut row_copy = row.clone();
                    row_copy.remove(i);
                    if check_row(&row_copy).is_none() {
                        println!(
                            "Row: {:?} is safe after removing {:?} [{:?}]",
                            row, row[i], index
                        );
                        now_safe = true;
                        break;
                    }
                }

                if now_safe {
                    safe_count += 1;
                } else {
                    println!(
                        "\tRow: {:?} is still unsafe after removing {:?} [{:?}]",
                        row, row[index as usize], index
                    );
                    unsafe_count += 1;
                }
            }
        }
    }
    (safe_count, unsafe_count)
}

fn main() -> Result<()> {
    let content = retrieve_data()?;
    let data = parse_data(&content)?;
    let (safe_count, unsafe_count) = summarise(&data, true);
    println!("Safe: {}, Unsafe: {}", safe_count, unsafe_count);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all_increasing() {
        let row = vec![1, 2, 3, 4, 5];
        assert_eq!(all_increasing(&row), 4);
    }

    #[test]
    fn test_all_decreasing() {
        let row = vec![5, 4, 3, 2, 1];
        assert_eq!(all_decreasing(&row), 4);
    }

    #[test]
    fn test_distance_ok() {
        let row = vec![1, 2, 3, 4, 5];
        assert_eq!(distance_ok(&row), 0);
    }

    #[test]
    fn test_summarise() {
        let data = vec![vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]];
        let (safe_count, unsafe_count) = summarise(&data);
        assert_eq!(safe_count, 2);
        assert_eq!(unsafe_count, 0);
    }

    #[test]
    fn test_expected_results() {
        let expected_safe = 4;
        let expected_unsafe = 2;

        let example_data = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];

        let (safe_count, unsafe_count) = summarise(&example_data);
        assert_eq!(safe_count, expected_safe);
        assert_eq!(unsafe_count, expected_unsafe);
    }
}
