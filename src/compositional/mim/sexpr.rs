// S-expression parser for m17n .mim specs. Ported from the khipro library
// (core/src/sexpr.cpp): atoms, lists, quoted strings and `;;` line comments.

/// A parsed S-expression node: either an atom (string token) or a list.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

impl Sexp {
    /// Returns the atom's string slice, or `None` for a list.
    pub(crate) fn as_atom(&self) -> Option<&str> {
        match self {
            Sexp::Atom(s) => Some(s),
            Sexp::List(_) => None,
        }
    }

    /// Returns the children of a list, or an empty slice for an atom.
    #[cfg(test)]
    pub(crate) fn as_list(&self) -> &[Sexp] {
        match self {
            Sexp::List(items) => items,
            Sexp::Atom(_) => &[],
        }
    }
}

pub(crate) struct SexprParser {
    text: Vec<char>,
    idx: usize,
}

impl SexprParser {
    pub(crate) fn new(text: &str) -> Self {
        SexprParser {
            text: text.chars().collect(),
            idx: 0,
        }
    }

    /// Parses all top level expressions in the input.
    pub(crate) fn parse_all(&mut self) -> Vec<Sexp> {
        let mut out = Vec::new();
        while self.skip_ws() {
            out.push(self.read_expr());
        }
        out
    }

    /// Skips whitespace and `;;` comments. Returns `false` at end of input.
    fn skip_ws(&mut self) -> bool {
        while self.idx < self.text.len() {
            let c = self.text[self.idx];
            if c.is_whitespace() {
                self.idx += 1;
                continue;
            }
            if c == ';' && self.idx + 1 < self.text.len() && self.text[self.idx + 1] == ';' {
                while self.idx < self.text.len() && self.text[self.idx] != '\n' {
                    self.idx += 1;
                }
                continue;
            }
            return true;
        }
        false
    }

    fn read_expr(&mut self) -> Sexp {
        // Caller guarantees `skip_ws` returned true, so a token is available.
        let c = self.text[self.idx];
        if c == '(' {
            self.idx += 1;
            let mut list = Vec::new();
            while self.skip_ws() && self.text[self.idx] != ')' {
                list.push(self.read_expr());
            }
            // Consume the closing paren if present. Mirrors the lenient
            // behaviour of the reference parser for well formed specs.
            if self.idx < self.text.len() && self.text[self.idx] == ')' {
                self.idx += 1;
            }
            return Sexp::List(list);
        }
        if c == '"' {
            return Sexp::Atom(self.read_string());
        }
        Sexp::Atom(self.read_atom())
    }

    fn read_string(&mut self) -> String {
        let mut out = String::new();
        self.idx += 1; // opening quote
        while self.idx < self.text.len() && self.text[self.idx] != '"' {
            if self.text[self.idx] == '\\' && self.idx + 1 < self.text.len() {
                let nxt = self.text[self.idx + 1];
                match nxt {
                    '\\' => out.push('\\'),
                    '"' => out.push('"'),
                    'n' => out.push('\n'),
                    't' => out.push('\t'),
                    'r' => out.push('\r'),
                    _ => out.push(nxt),
                }
                self.idx += 2;
                continue;
            }
            out.push(self.text[self.idx]);
            self.idx += 1;
        }
        if self.idx < self.text.len() {
            self.idx += 1; // closing quote
        }
        out
    }

    fn read_atom(&mut self) -> String {
        let start = self.idx;
        while self.idx < self.text.len() {
            let c = self.text[self.idx];
            if c.is_whitespace() || c == '(' || c == ')' {
                break;
            }
            self.idx += 1;
        }
        self.text[start..self.idx].iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoms_and_lists() {
        let mut p = SexprParser::new("(map (shor (\"o\" \"অ\")))");
        let root = p.parse_all();
        assert_eq!(root.len(), 1);
        let top = root[0].as_list();
        assert_eq!(top[0].as_atom(), Some("map"));
        let shor = top[1].as_list();
        assert_eq!(shor[0].as_atom(), Some("shor"));
        let entry = shor[1].as_list();
        assert_eq!(entry[0].as_atom(), Some("o"));
        assert_eq!(entry[1].as_atom(), Some("অ"));
    }

    #[test]
    fn test_comments_skipped() {
        let mut p = SexprParser::new(";; comment line\n(a b) ;; trailing\n(c)");
        let root = p.parse_all();
        assert_eq!(root.len(), 2);
        assert_eq!(root[0].as_list()[0].as_atom(), Some("a"));
        assert_eq!(root[1].as_list()[0].as_atom(), Some("c"));
    }

    #[test]
    fn test_string_escapes() {
        let mut p = SexprParser::new("(\"a\\\"b\" \"c\\\\d\")");
        let root = p.parse_all();
        let list = root[0].as_list();
        assert_eq!(list[0].as_atom(), Some("a\"b"));
        assert_eq!(list[1].as_atom(), Some("c\\d"));
    }
}
