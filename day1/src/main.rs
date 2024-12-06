use anyhow::Result;
use std::{fs::File, io::Read};

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut total_similarity = 0;
    for v in left {
        let count = right.iter().filter(|x| v == **x).count();
        total_similarity += v * count as i32;
    }

    total_similarity
}

fn calculate_distance(list: Vec<(i32, i32)>) -> i32 {
    let mut total_distance = 0;

    for (l, r) in list {
        if l > r {
            total_distance += l - r;
        } else {
            total_distance += r - l;
        }
    }

    total_distance
}

fn process_data(content: &str) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for l in content.lines() {
        let vals: Vec<&str> = l.split("   ").collect();
        left.push(vals[0].parse::<i32>()?);
        right.push(vals[1].parse::<i32>()?);
    }

    left.sort();
    right.sort();

    Ok((left, right))
}

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day1.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<()> {
    let content = retrieve_data()?;
    let (left, right) = process_data(&content)?;
    let combined = left
        .clone()
        .into_iter()
        .zip(right.clone().into_iter())
        .collect();
    let total_distance = calculate_distance(combined);
    let total_similarity = calculate_similarity(left, right);
    println!("{}", total_distance);
    println!("{}", total_similarity);
    Ok(())
}
