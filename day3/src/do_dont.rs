use crate::token::{iter_closing_brace, iter_comma, iter_digit, iter_whitespace, Token, TokenType};
use anyhow::{anyhow, Result};

pub struct Do {}
pub struct Dont {}

impl Do {
    pub fn from_token_string(token_string: &str) -> Result<Do> {
        let mut idx = 3; // do(
        if token_string.len() > idx && token_string[0..idx].as_bytes() != "do(".as_bytes() {
            Err(anyhow!("No do found"))?;
        }
        let token_string = token_string.as_bytes();
        idx = iter_whitespace(token_string, idx);
        idx = iter_closing_brace(token_string, idx)?;
        Ok(Do {})
    }
}

impl Token for Do {
    fn get_type(&self) -> TokenType {
        TokenType::Do
    }

    fn get_text(&self) -> String {
        format!("do()")
    }

    fn eval(&self) -> i32 {
        1
    }

    fn from_token_string(token_string: &str) -> Result<Do> {
        Do::from_token_string(token_string)
    }
}
impl Dont {
    pub fn from_token_string(token_string: &str) -> Result<Dont> {
        let mut idx = 6; // don't(
        if token_string.len() > idx && token_string[0..idx].as_bytes() != "don't(".as_bytes() {
            Err(anyhow!("No do found"))?;
        }

        let token_string = token_string.as_bytes();
        idx = iter_whitespace(token_string, idx);
        iter_closing_brace(token_string, idx)?;
        Ok(Dont {})
    }
}

impl Token for Dont {
    fn get_type(&self) -> TokenType {
        TokenType::Dont
    }

    fn get_text(&self) -> String {
        format!("do()")
    }

    fn eval(&self) -> i32 {
        0
    }

    fn from_token_string(token_string: &str) -> Result<Dont> {
        Dont::from_token_string(token_string)
    }
}
