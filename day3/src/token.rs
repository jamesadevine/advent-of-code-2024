use anyhow::{anyhow, Result};

pub trait Token {
    fn get_type(&self) -> TokenType;
    fn get_text(&self) -> String;
    fn eval(&self) -> i32;
    fn from_token_string(token_string: &str) -> Result<Self>
    where
        Self: Sized;
}

pub enum TokenType {
    Mul,
    Do,
    Dont,
}

pub fn iter_digit(token_string: &[u8], idx: usize) -> Result<usize> {
    if idx > token_string.len() - 1 {
        Err(anyhow!("String index out of bounds"))?
    }

    let orig = idx.clone();
    let mut idx = idx;
    while idx < token_string.len() && token_string[idx].is_ascii_digit() {
        idx += 1;
    }
    if orig == idx {
        Err(anyhow!(
            "No digit found for index {} in {:?}",
            orig,
            String::from_utf8(token_string[idx..].to_vec()).unwrap()
        ))
    } else {
        Ok(idx)
    }
}

pub fn iter_comma(token_string: &[u8], idx: usize) -> Result<usize> {
    if idx > token_string.len() - 1 || token_string[idx] != b',' {
        Err(anyhow!("No comma found for index {}", idx))
    } else {
        Ok(idx + 1)
    }
}

pub fn iter_closing_brace(token_string: &[u8], idx: usize) -> Result<usize> {
    if idx > token_string.len() - 1 || token_string[idx] != b')' {
        Err(anyhow!("No closing brace found for index {}", idx))
    } else {
        Ok(idx + 1)
    }
}

pub fn iter_whitespace(token_string: &[u8], idx: usize) -> usize {
    let mut idx = idx;
    while idx < token_string.len() && token_string[idx].is_ascii_whitespace() {
        idx += 1;
    }
    idx
}
