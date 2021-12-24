use std::{fmt::Display, str::from_utf8};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next(&mut self) -> Option<u8> {
        if self.cursor < self.source.len() {
            let byte = self.source[self.cursor];
            self.next_pos();
            Some(byte)
        } else {
            return None;
        }
    }

    fn next_pos(&mut self) {
        self.cursor += 1;
    }

    fn lookahead(&mut self) -> Option<u8> {
        let idx = self.cursor;
        if idx < self.source.len() {
            Some(self.source[idx])
        } else {
            None
        }
    }

    pub fn tokenize(&mut self) -> Option<Token> {
        loop {
            if let Some(c) = self.next() {
                return match c {
                    b'0'..=b'9' => self.read_number(c),
                    b'a'..=b'z' | b'A'..=b'Z' => self.read_label(c),
                    b'\r' | b'\n' => self.read_eol(),
                    b'+' | b'-' | b'*' | b'\\' => self.read_operator(c),
                    _ => continue,
                };
            } else {
                // return Some(Token::EOF);
                return None;
            }
        }
    }

    fn read_number(&mut self, first: u8) -> Option<Token> {
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.lookahead() {
            match c {
                b'0'..=b'9' => buf.push(c as char),
                _ => break,
            }
            self.next();
        }
        match buf.parse::<f64>() {
            Ok(i) => Some(Token::Number(i)),
            Err(_) => None,
        }
    }

    fn read_label(&mut self, first: u8) -> Option<Token> {
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.lookahead() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' => buf.push(c as char),
                _ => break,
            }
            self.next();
        }
        Some(Token::Identifier(buf))
    }

    fn read_eol(&mut self) -> Option<Token> {
        Some(Token::Eol)
    }

    fn read_operator(&self, op: u8) -> Option<Token> {
        let s = String::from(op as char);
        //TODO: 处理“-”开头的的负数
        //TODO: 处理“/” 开头的comment或者regexp
        
        Some(Token::Operator(s))
    }
}

impl Display for Lexer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.source).unwrap())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_read_number() {
        let n = &[b'1', b'2', b'3'];
        println!("{:?}", n);
        let mut res = 0u32;
        res = (res * 10) + (n[0] - b'0') as u32;
        res = (res * 10) + (n[1] - b'0') as u32;
        res = (res * 10) + (n[2] - b'0') as u32;
        println!("{:?}", res);
    }

    #[test]
    fn temp() {}
}
