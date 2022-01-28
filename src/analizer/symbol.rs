// 没用到
pub enum SymbolKind {
    Variable,
    Function,
    // BuiltinKind,  // TODO::
}

#[derive(Hash, PartialEq, Eq)]
pub struct Symbol {
    name: String,
    kind: SymbolKind,
}

impl Symbol {
    pub fn new(name: String, kind: SymbolKind) -> Self {
        Self { name, kind }
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: <{}>]", self.name, self.kind)
    }
}
impl std::fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolKind::Variable => write!(f, "variable"),
            SymbolKind::Function => write!(f, "function"),
        }
    }
}
