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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Span<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}
