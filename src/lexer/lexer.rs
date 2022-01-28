use crate::{
    error::ParserError,
    parser::ParseResult,
    position::Pos,
    token::{Keyword, Operator, Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub filename: &'a str,
    source: &'a [u8],
    cursor: usize,
    ln: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8], filename: &'a str) -> Self {
        Self {
            filename,
            source,
            cursor: 0,
            ln: 1,
            col: 1,
        }
    }

    // some ast start from col= 0, and some start from col =1
    // vscode editor start from col = 1
    pub fn pos(&mut self) -> Pos {
        Pos::new(self.ln, self.col)
        // Pos::new(self.ln, self.col - 1)
    }

    fn advance(&mut self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }
        let byte = self.source[self.cursor];
        self.next_pos();
        Some(byte)
    }

    pub fn is_eof(&mut self) -> bool {
        self.cursor >= self.source.len()
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
        match self.advance() {
            Some(c) => {
                return Ok(match c {
                    b'0'..=b'9' => self.read_number(c, start)?,
                    b'\r' | b'\n' => self.read_eol(start),
                    b'"' => self.read_string(c, start)?,
                    b'a'..=b'z' | b'A'..=b'Z' => self.read_identifier(c, start)?,
                    b'/' => {
                        if Some(b'/') == self.peek() {
                            self.skip_comment()?
                        } else {
                            self.read_operator(c, start)?
                        }
                    }
                    b'+' | b'-' | b'*' | b'=' | b'>' | b'<' | b'!' | b'|' | b'&' => {
                        self.read_operator(c, start)?
                    }
                    b',' => Token::new(TokenKind::Comma, ",".to_string(), start, self.pos()),
                    b';' => Token::new(TokenKind::Semi, ";".to_string(), start, self.pos()),
                    b'{' => Token::new(TokenKind::BraceOpen, "{".to_string(), start, self.pos()),
                    b'}' => Token::new(TokenKind::BraceClose, "}".to_string(), start, self.pos()),
                    b'(' => Token::new(TokenKind::ParenOpen, "(".to_string(), start, self.pos()),
                    b')' => Token::new(TokenKind::ParenClose, ")".to_string(), start, self.pos()),
                    _ => {
                        return Err(ParserError::invalid_charactor(
                            self.filename,
                            c as char,
                            start,
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
        let mut has_point = false;
        while let Some(c) = self.peek() {
            match c {
                b'0'..=b'9' => buf.push(c as char),
                b'.' => {
                    if !has_point {
                        has_point = true;
                        buf.push('.')
                    } else {
                        return Err(ParserError::invalid_charactor(
                            self.filename,
                            '.',
                            self.pos(),
                        ));
                    }
                }
                _ => break,
            }
            self.advance();
        }
        Ok(Token::new(TokenKind::Number, buf, start, self.pos()))
    }

    fn read_string(&mut self, _c: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = vec![];
        while let Some(c) = self.advance() {
            match c {
                b'"' => break,
                b'\n' => self.newline(),
                _ => buf.push(c),
            }
        }

        // TODO: excaped , unicode..
        let s = String::from_utf8(buf).expect("invalid utf8");
        Ok(Token::new(TokenKind::String, s, start, self.pos()))
    }

    // 字母开头，可以包含数字和下划线
    fn read_identifier(&mut self, first: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => buf.push(c as char),
                _ => break,
            }
            self.advance();
        }

        if &buf == "true" || &buf == "false" {
            Ok(Token::new(TokenKind::Boolean, buf, start, self.pos()))
        } else if &buf == "null" {
            Ok(Token::new(TokenKind::Null, buf, start, self.pos()))
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
                    self.advance();
                } else {
                    buf.push(x as char)
                }
            }
            x @ b'|' => {
                if self.peek() == Some(b'|') {
                    buf.push(x as char);
                    buf.push('|');
                    self.advance();
                } else {
                    return Err(ParserError::invalid_charactor(
                        self.filename,
                        '.',
                        self.pos(),
                    ));
                }
            }
            x @ b'&' => {
                if self.peek() == Some(b'&') {
                    buf.push(x as char);
                    buf.push('&');
                    self.advance();
                } else {
                    return Err(ParserError::invalid_charactor(
                        self.filename,
                        '.',
                        self.pos(),
                    ));
                }
            }
            _ => unimplemented!(),
        }

        let op = Operator::from(&buf);
        Ok(Token::new(TokenKind::Operator(op), buf, start, self.pos()))
    }

    fn skip_comment(&mut self) -> ParseResult<Token> {
        while let Some(c) = self.peek() {
            match c {
                b'\n' | b'\r' => break,
                _ => self.advance(),
            };
        }
        self.next()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            match c {
                b' ' | b'\t' => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    // 多个空行，仅返回一个\n，用来判断statement end
    fn read_eol(&mut self, start: Pos) -> Token {
        self.newline();

        while let Some(c) = self.peek() {
            match c {
                b'\n' | b'\r' => {
                    self.advance();
                    self.newline();
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
                    TokenKind::Eol => continue,
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
