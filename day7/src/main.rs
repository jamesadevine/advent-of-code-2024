use anyhow::Result;
use std::{
    fmt::{Debug, Display},
    fs::File,
    io::Read,
};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day7\\day7.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

struct OpResult {
    result: usize,
}

impl OpResult {
    fn from_string(s: &str) -> Result<OpResult> {
        let result = s.parse::<usize>()?;
        Ok(OpResult { result })
    }
}

#[derive(Clone)]
struct OpComponent {
    value: usize,
}

impl OpComponent {
    fn from_string(s: &str) -> Result<OpComponent> {
        let value = s.parse::<usize>()?;
        Ok(OpComponent { value })
    }
}

impl ToString for OpComponent {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}
impl Debug for OpComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct Operation {
    result: OpResult,
    numbers: Vec<OpComponent>,
    is_valid: bool,
}

static OPERATORS: [char; 3] = ['+', '*', '|'];

impl Operation {
    fn from_string(s: &str) -> Result<Operation> {
        let parts: Vec<&str> = s.split(":").collect();
        let result = OpResult::from_string(parts[0])?;

        let numbers: Vec<OpComponent> = parts[1]
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| OpComponent::from_string(x).unwrap())
            .collect::<Vec<OpComponent>>();

        Ok(Operation {
            result,
            numbers,
            is_valid: false,
        })
    }

    fn combine(&self, comp1: &OpComponent, comp2: &OpComponent) -> Result<usize> {
        Ok((comp1.to_string() + &comp2.to_string()).parse::<usize>()?)
    }

    fn recursive_calc(&self, component_numbers: &[OpComponent], idx: usize, result: usize) -> bool {
        if idx == component_numbers.len() {
            if result == self.result.result {
                return true;
            } else {
                return false;
            }
        }

        let num = component_numbers[idx].value;

        for op in OPERATORS.iter() {
            match op {
                '+' => match self.recursive_calc(&component_numbers, idx + 1, result + num) {
                    true => return true,
                    false => continue,
                },
                '*' => {
                    let mut result = result;
                    if result == 0 {
                        result = 1;
                    }
                    match self.recursive_calc(&component_numbers, idx + 1, result * num) {
                        true => return true,
                        false => continue,
                    }
                }
                '|' => {
                    let new_num = self
                        .combine(&OpComponent { value: result }, &component_numbers[idx])
                        .unwrap();

                    // start a new search with the new list
                    match self.recursive_calc(&component_numbers, idx + 1, new_num) {
                        true => return true,
                        false => continue,
                    }
                }
                _ => {
                    panic!("Invalid operator");
                }
            }
        }
        false
    }

    fn evaluate(&mut self) {
        self.is_valid = self.recursive_calc(&self.numbers, 0, 0);
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation: {} = {:?}", self.result.result, self.numbers)
    }
}

fn main() {
    let data = retrieve_data().unwrap();
    let mut operations: Vec<Operation> = data
        .lines()
        .map(|x| Operation::from_string(x).unwrap())
        .collect::<Vec<Operation>>();

    let mut total = 0;
    for op in operations.iter_mut() {
        op.evaluate();
        if op.is_valid {
            total += op.result.result;
        }
        println!("Operation: {} is valid: {}", op, op.is_valid);
    }
    println!("Total: {}", total);
}
