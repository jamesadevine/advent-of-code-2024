use crate::token::{
    self, iter_closing_brace, iter_comma, iter_digit, iter_whitespace, Token, TokenType,
};
use anyhow::{anyhow, Result};

pub struct Mul {
    pub x: i32,
    pub y: i32,
}

impl Token for Mul {
    fn get_type(&self) -> TokenType {
        TokenType::Mul
    }

    fn get_text(&self) -> String {
        format!("mul({}, {})", self.x, self.y)
    }

    fn eval(&self) -> i32 {
        self.x * self.y
    }

    fn from_token_string(token_string: &str) -> Result<Mul> {
        Mul::from_token_string(token_string)
    }
}

impl Mul {
    pub fn new(x: i32, y: i32) -> Mul {
        Mul { x, y }
    }

    pub fn from_token_string(token_string: &str) -> Result<Mul> {
        let mut idx = 4; // mul(
        if token_string.len() > idx && token_string[0..idx].as_bytes() != "mul(".as_bytes() {
            Err(anyhow!("No mul found"))?;
        }

        let token_string = token_string.as_bytes();
        let mut x_start_ind = -1;
        let mut x_end_ind = -1;
        let mut y_start_ind = -1;
        let mut y_end_ind = -1;

        idx = iter_whitespace(token_string, idx);
        x_start_ind = idx.clone() as i32;
        idx = iter_digit(token_string, idx)?;
        x_end_ind = idx.clone() as i32;
        idx = iter_whitespace(token_string, idx);
        idx = iter_comma(token_string, idx)?;
        idx = iter_whitespace(token_string, idx);
        y_start_ind = idx.clone() as i32;
        idx = iter_digit(token_string, idx)?;
        y_end_ind = idx.clone() as i32;
        idx = iter_whitespace(token_string, idx);
        idx = iter_closing_brace(token_string, idx)?;

        let x = String::from_utf8(token_string[x_start_ind as usize..x_end_ind as usize].to_vec())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let y = String::from_utf8(token_string[y_start_ind as usize..y_end_ind as usize].to_vec())
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Ok(Mul { x, y })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_mul_parse_simple() {
        let input = "mul(2,4)";
        let mul = Mul::from_token_string(input).unwrap();
        assert_eq!(mul.x, 2);
        assert_eq!(mul.y, 4);
    }
}
