use crate::{
    token::{Keyword, Token, TokenKind},
    token::{Loc, Operator, Pos},
    Error, ParseResult,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub filename: &'a str,
    source: &'a [u8],
    cursor: usize,
    ln: usize,
    col: usize,
    pub loc: Loc,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8], filename: &'a str) -> Self {
        Self {
            filename,
            source,
            cursor: 0,
            ln: 1,
            col: 1,
            loc: Loc::new(Pos::new(1, 1), Pos::new(1, 1)),
        }
    }

    pub fn pos(&mut self) -> Pos {
        // Pos::new(self.ln, self.col)
        Pos::new(self.ln, self.col - 1)
    }

    fn next_byte(&mut self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }
        let byte = self.source[self.cursor];
        self.next_pos();
        Some(byte)
    }

    pub fn is_eof(&mut self) -> bool {
        self.cursor == self.source.len()
    }

    fn next_pos(&mut self) {
        self.cursor += 1;
        self.col += 1;
    }

    fn newline(&mut self) {
        self.ln += 1;
        self.col = 1;
    }

    fn peek(&mut self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }
        Some(self.source[self.cursor])
    }

    pub fn next(&mut self) -> ParseResult<Token> {
        self.skip_whitespace();
        let start = self.pos();
        match self.next_byte() {
            Some(c) => {
                return Ok(match c {
                    b'0'..=b'9' => self.read_number(c, start)?,
                    b'\r' | b'\n' => self.read_eol(start),
                    b'"' => self.read_string(c, start)?,
                    b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(c, start)?,
                    b'+' | b'-' | b'*' | b'/' | b'=' | b'>' | b'<' | b'!' => {
                        self.read_operator(c, start)?
                    }
                    b',' => Token::new(TokenKind::Comma, ",".to_string(), start, self.pos()),
                    b';' => Token::new(TokenKind::Semi, ";".to_string(), start, self.pos()),
                    b'{' => Token::new(TokenKind::BraceOpen, "{".to_string(), start, self.pos()),
                    b'}' => Token::new(TokenKind::BraceClose, "}".to_string(), start, self.pos()),
                    b'(' => Token::new(TokenKind::ParenOpen, "(".to_string(), start, self.pos()),
                    b')' => Token::new(TokenKind::ParenClose, ")".to_string(), start, self.pos()),
                    _ => {
                        return Err(Error::invalid_charactor(
                            self.filename,
                            c as char,
                            self.pos(),
                        ));
                    }
                })
            }
            None => {
                return Ok(Token::new(
                    TokenKind::Eof,
                    "EndOfFile".to_string(),
                    start,
                    self.pos(),
                ))
            }
        }
    }

    fn read_number(&mut self, first: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.peek() {
            match c {
                b'0'..=b'9' => buf.push(c as char),
                _ => break,
            }
            self.next_byte();
        }
        Ok(Token::new(TokenKind::Number, buf, start, self.pos()))
    }

    fn read_string(&mut self, _c: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = vec![];
        while let Some(c) = self.next_byte() {
            match c {
                b'"' => break,
                _ => buf.push(c),
            }
        }

        // TODO: excaped , unicode..
        let s = String::from_utf8(buf).expect("invalid utf8");
        Ok(Token::new(TokenKind::String, s, start, self.pos()))
    }

    fn read_identifier(&mut self, first: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' => buf.push(c as char),
                _ => break,
            }
            self.next_byte();
        }

        if &buf == "true" || &buf == "false" {
            Ok(Token::new(TokenKind::Boolean, buf, start, self.pos()))
        } else {
            match Keyword::from_str(&buf) {
                Some(key) => Ok(Token::new(TokenKind::Keyword(key), buf, start, self.pos())),
                None => Ok(Token::new(TokenKind::Identifier, buf, start, self.pos())),
            }
        }
    }

    /**
     *  '+' | '-' | '*' | '/' | '=' | '>' | '<' | '!'
     *  '==' | '>=' | '<=' | '!='
     */
    fn read_operator(&mut self, op: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = String::new();

        match op {
            x @ (b'+' | b'-' | b'*' | b'/') => buf.push(x as char),
            x @ (b'=' | b'>' | b'<' | b'!') => {
                if self.peek() == Some(b'=') {
                    buf.push(x as char);
                    buf.push('=');
                    self.next_byte();
                } else {
                    buf.push(x as char)
                }
            }
            _ => unimplemented!(),
        }

        let op = Operator::from_str(&buf);
        Ok(Token::new(TokenKind::Operator(op), buf, start, self.pos()))
    }

    // 多个空行，仅返回一个\n，用来判断auto semi insertion
    fn read_eol(&mut self, start: Pos) -> Token {
        self.newline();

        while let Some(c) = self.peek() {
            match c {
                b'\n' | b'\r' => {
                    self.newline();
                    self.next_byte();
                }
                _ => break,
            }
        }

        Token::new(TokenKind::Eol, "EndOfLine".to_string(), start, self.pos())
    }

    pub fn log(&mut self) {
        loop {
            match self.next() {
                Ok(token) => match token.kind {
                    // TokenKind::Eol => continue,
                    TokenKind::Eof => {
                        break;
                    }
                    _ => println!("{:#}", token),
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                    break;
                }
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b' ' | b'\t' => {
                    self.next_byte();
                }
                _ => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_read_operator() {
        let s = "+ - * / = ! == >= <= > <";
        let mut lex = Lexer::new(s.as_bytes(), "test");
        lex.log();
    }
}
