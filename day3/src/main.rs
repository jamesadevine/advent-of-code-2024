mod do_dont;
mod mul;
mod token;
use anyhow::Result;
use do_dont::{Do, Dont};
use mul::Mul;
use token::{Token, TokenType};

use std::{fs::File, io::Read};

fn retrieve_data() -> Result<String> {
    let mut f = File::open("C:\\software\\adventofcode24\\day3\\day3.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn parse(input: &str) -> Vec<Box<dyn Token>> {
    let mut tokens: Vec<Box<dyn Token>> = Vec::new();
    for (idx, _) in input.as_bytes().iter().enumerate() {
        match Mul::from_token_string(&input[idx..]) {
            Ok(mul) => tokens.push(Box::new(mul)),
            Err(_) => (),
        }
        match Do::from_token_string(&input[idx..]) {
            Ok(do_) => tokens.push(Box::new(do_)),
            Err(_) => (),
        }
        match Dont::from_token_string(&input[idx..]) {
            Ok(dont) => tokens.push(Box::new(dont)),
            Err(_) => (),
        }
    }
    tokens
}

fn main() {
    let data = retrieve_data().unwrap();
    let tokens = parse(&data);

    let mut result: i32 = 0;
    let mut do_ = true;
    for token in &tokens {
        match token.get_type() {
            TokenType::Mul => {
                if do_ {
                    result += token.eval()
                }
            }
            TokenType::Do => {
                do_ = true;
            }
            TokenType::Dont => {
                do_ = false;
            }
        }

        println!("{}", token.get_text());
    }

    println!("{}", result);
}
