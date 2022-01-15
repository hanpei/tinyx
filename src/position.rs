#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub ln: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(ln: usize, col: usize) -> Self {
        Self { ln, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loc {
    pub start: Pos,
    pub end: Pos,
}

impl Loc {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub filename: String,
    pub loc: Loc,
}

impl Span {
    pub fn new(filename: String, loc: Loc) -> Self {
        Self { filename, loc }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithSpan<T> {
    pub value: T,
    pub filename: String,
    pub loc: Loc,
}

impl<T> WithSpan<T> {
    pub fn new(value: T, filename: String, loc: Loc) -> Self {
        Self {
            value,
            filename,
            loc,
        }
    }
    pub fn span(&self) -> Span {
        Span::new(self.filename.to_string(), self.loc)
    }
}
