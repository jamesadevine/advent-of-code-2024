use anyhow::Result;
use std::{fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day5\\day5.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

#[derive(Clone)]
struct PageOrder {
    order: Vec<usize>,
}

impl PageOrder {
    fn from_line(line: &str) -> Result<Self> {
        if !line.contains(",") {
            Err(anyhow::anyhow!("Invalid order: {}", line))?
        }

        let order: Vec<usize> = line.split(",").map(|x| x.parse().unwrap()).collect();
        Ok(PageOrder { order })
    }

    fn middle(&self) -> usize {
        self.order[self.order.len() / 2]
    }
}

#[derive(Clone)]
struct OrderingRule {
    page: usize,
    before: usize,
    text: String,
}

impl OrderingRule {
    fn from_line(line: &str) -> Result<Self> {
        let parts: Vec<&str> = line.split("|").collect();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid rule: {}", line));
        }

        Ok(OrderingRule {
            page: parts[0].parse()?,
            before: parts[1].parse()?,
            text: line.to_string(),
        })
    }

    fn is_valid(&self, order: &Vec<usize>) -> bool {
        let page_pos = order.iter().position(|&x| x == self.page);
        let before_pos = order.iter().position(|&x| x == self.before);

        if page_pos.is_none() || before_pos.is_none() {
            return true;
        }
        page_pos.unwrap() < before_pos.unwrap()
    }
}

fn find_applicable_rules(rules: &Vec<OrderingRule>, order: &Vec<usize>) -> Vec<OrderingRule> {
    rules
        .iter()
        .filter(|rule| order.contains(&rule.page))
        .map(|rule| rule.clone())
        .collect()
}

fn apply_rules(rules: &Vec<OrderingRule>, order: &Vec<usize>) -> Option<OrderingRule> {
    for rule in rules {
        if !rule.is_valid(order) {
            return Some(rule.to_owned().clone());
        }
    }
    None
}

fn main() -> Result<()> {
    let data = retrieve_data().unwrap();
    let mut rules = Vec::new();
    let mut orders = Vec::new();
    for line in data.lines() {
        match OrderingRule::from_line(line) {
            Ok(rule) => rules.push(rule),
            Err(_) => (),
        }
        match PageOrder::from_line(line) {
            Ok(order) => orders.push(order),
            Err(_) => (),
        }
    }

    let mut incorrect_orders = Vec::new();
    let mut total_from_correct = 0;
    for order in &orders {
        let applied_rule = apply_rules(&rules, &order.order);

        if applied_rule.is_none() {
            total_from_correct += order.middle();
            continue;
        }

        println!(
            "Invalid order: {:?} from rule: {:?} ",
            order.order,
            applied_rule.unwrap().text
        );

        incorrect_orders.push(order);
    }

    println!(
        "Total middle values from correct order: {}",
        total_from_correct
    );

    let mut middle_sum_incorrect = 0;

    for incorrect in &incorrect_orders {
        let rules = find_applicable_rules(&rules, &incorrect.order);
        let mut new_order = incorrect.to_owned().clone();

        loop {
            match apply_rules(&rules, &new_order.order) {
                Some(rule) => {
                    let page_pos = new_order
                        .order
                        .iter()
                        .position(|&x| x == rule.page)
                        .unwrap();
                    let before_pos = new_order
                        .order
                        .iter()
                        .position(|&x| x == rule.before)
                        .unwrap();
                    new_order.order.swap(page_pos, before_pos);
                }
                None => {
                    println!("Corrected order: {:?}", new_order.order);
                    middle_sum_incorrect += new_order.middle();
                    break;
                }
            }
        }
    }

    println!(
        "Total middle values from incorrect order: {}",
        middle_sum_incorrect
    );

    Ok(())
}
