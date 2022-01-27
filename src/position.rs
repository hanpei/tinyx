#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Pos {
    pub ln: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(ln: usize, col: usize) -> Self {
        Self { ln, col }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Loc {
    pub start: Pos,
    pub end: Pos,
}

impl Loc {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Span {
    pub filename: String,
    pub loc: Loc,
}

impl Span {
    pub fn new(filename: String, loc: Loc) -> Self {
        Self { filename, loc }
    }
}

#[derive(Clone, PartialEq)]
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

// impl std::fmt::Debug for Span {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "")
//     }
// }
impl<T: std::fmt::Debug> std::fmt::Debug for WithSpan<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
