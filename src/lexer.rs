use crate::{
    token::{Token, TokenKind},
    Error, ParseResult, ast::Operator,
};

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub ln: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(ln: usize, col: usize) -> Self {
        Self { ln, col }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub start: Pos,
    pub end: Pos,
}

impl Loc {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }
}

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
                    b'a'..=b'z' | b'A'..=b'Z' => self.read_ident(c, start)?,
                    b'+' | b'-' | b'*' | b'/' => self.read_operator(c, start)?,
                    b';' => Token::new(TokenKind::Semi, ";".to_string(), start, self.pos()),
                    b'{' => Token::new(TokenKind::BraceOpen, "{".to_string(), start, self.pos()),
                    b'}' => Token::new(TokenKind::BraceClose, "}".to_string(), start, self.pos()),
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
                    self.pos(),
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
        // match buf.parse::<f64>() {
        //     Err(_e) => Err(Error::parse_number_error(self.filename, &buf, self.pos())),
        // }
    }

    fn read_string(&mut self, _c: u8, start: Pos) -> ParseResult<Token> {
        let mut buf = vec![];
        while let Some(c) = self.next_byte() {
            match c {
                b'"' => break,
                _ => buf.push(c),
            }
        }

        let s = String::from_utf8(buf).expect("invalid utf8");
        Ok(Token::new(TokenKind::String, s, start, self.pos()))
    }

    fn read_ident(&mut self, first: u8, start: Pos) -> ParseResult<Token> {
        // TODO: Identifier需要确定
        let mut buf = String::new();
        buf.push(first as char);
        while let Some(c) = self.peek() {
            match c {
                b'a'..=b'z' | b'A'..=b'Z' => buf.push(c as char),
                _ => break,
            }
            self.next_byte();
        }

        Ok(Token::new(TokenKind::Identifier, buf, start, self.pos()))
    }

    fn read_operator(&mut self, op: u8, start: Pos) -> ParseResult<Token> {
        let s = String::from(op as char);
        //TODO: 处理“-”开头的的负数
        //TODO: 处理“/” 开头的comment或者regexp
        Ok(Token::new(TokenKind::Operator, s, start, self.pos()))
    }

    fn read_eol(&mut self, start: Pos) -> Token {
        self.newline();
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
