// m17n .mim specification parser. Ported from the khipro library
// (core/src/spec.h and core/src/spec.cpp). Supports the subset used by Khipro:
// `map` and `state` blocks with set/insert/delete/move/shift/commit/undo/cond.

use std::collections::HashMap;

use super::sexpr::{Sexp, SexprParser};

#[derive(Debug, Clone)]
pub(crate) enum Condition {
    /// Always matches (the `1` condition or a `t`/default branch).
    Always,
    /// `(= VAR value)`
    Equals { variable: String, value: i64 },
    /// `(& a b)`
    And(Box<Condition>, Box<Condition>),
    /// `(| a b)`
    Or(Box<Condition>, Box<Condition>),
}

#[derive(Debug, Clone)]
pub(crate) struct CondBranch {
    pub(crate) condition: Condition,
    pub(crate) actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub(crate) enum Action {
    /// `(set VAR value)` where value is a literal or another variable name.
    Set { name: String, value: String },
    /// `(insert "text")`
    Insert(String),
    /// `(delete @-N)` deletes `n` characters before the cursor.
    Delete { n: i64 },
    /// `(move @-N)` or `(move @>)`.
    Move { n: i64, to_end: bool },
    /// `(shift state-name)`
    Shift(String),
    /// `(commit)` — a no-op for output purposes.
    Commit,
    /// `(undo)`
    Undo,
    /// `(cond (branch...) ...)`
    Cond(Vec<CondBranch>),
}

#[derive(Debug, Clone)]
pub(crate) struct MapEntry {
    pub(crate) key: String,
    pub(crate) actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub(crate) struct StateRule {
    pub(crate) map_name: String,
    pub(crate) actions: Vec<Action>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct StateDef {
    pub(crate) entry_actions: Vec<Action>,
    pub(crate) rules: Vec<StateRule>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct Spec {
    pub(crate) maps: HashMap<String, Vec<MapEntry>>,
    pub(crate) states: HashMap<String, StateDef>,
}

fn parse_int_or_default(s: &str, fallback: i64) -> i64 {
    s.parse::<i64>().unwrap_or(fallback)
}

fn parse_condition(node: &Sexp) -> Option<Condition> {
    let list = match node {
        Sexp::List(items) if !items.is_empty() => items,
        _ => return None,
    };
    let head = list[0].as_atom()?;

    match head {
        "1" => Some(Condition::Always),
        "=" => {
            if list.len() >= 3 {
                if let (Some(var), Some(val)) = (list[1].as_atom(), list[2].as_atom()) {
                    return Some(Condition::Equals {
                        variable: var.to_string(),
                        value: parse_int_or_default(val, 0),
                    });
                }
            }
            None
        }
        "&" | "|" => {
            if list.len() >= 3 {
                let left = parse_condition(&list[1])?;
                let right = parse_condition(&list[2])?;
                if head == "&" {
                    Some(Condition::And(Box::new(left), Box::new(right)))
                } else {
                    Some(Condition::Or(Box::new(left), Box::new(right)))
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_action(node: &Sexp) -> Option<Action> {
    let list = match node {
        Sexp::List(items) if !items.is_empty() => items,
        _ => return None,
    };
    let head = list[0].as_atom()?;

    match head {
        "set" => {
            if list.len() >= 3 {
                if let (Some(name), Some(value)) = (list[1].as_atom(), list[2].as_atom()) {
                    return Some(Action::Set {
                        name: name.to_string(),
                        value: value.to_string(),
                    });
                }
            }
            None
        }
        "insert" => {
            let mut text = list.get(1)?.as_atom()?.to_string();
            // Character literals are written as `?<char>` in m17n.
            if text.starts_with('?') {
                text.remove(0);
            }
            Some(Action::Insert(text))
        }
        "delete" => {
            let token = list.get(1)?.as_atom()?;
            if let Some(count) = token.strip_prefix("@-") {
                let n = if count.is_empty() {
                    1
                } else {
                    parse_int_or_default(count, 1)
                };
                Some(Action::Delete { n })
            } else {
                None
            }
        }
        "move" => {
            let token = list.get(1)?.as_atom()?;
            if token == "@>" {
                Some(Action::Move {
                    n: 0,
                    to_end: true,
                })
            } else if token == "@-" {
                Some(Action::Move {
                    n: -1,
                    to_end: false,
                })
            } else {
                token.strip_prefix("@-").map(|count| Action::Move {
                    n: -parse_int_or_default(count, 1),
                    to_end: false,
                })
            }
        }
        "shift" => {
            let name = list.get(1)?.as_atom()?;
            Some(Action::Shift(name.to_string()))
        }
        "commit" => Some(Action::Commit),
        "undo" => Some(Action::Undo),
        "cond" => {
            let mut branches = Vec::new();
            for branch in &list[1..] {
                let items = match branch {
                    Sexp::List(items) if !items.is_empty() => items,
                    _ => continue,
                };
                let condition = if items[0].as_atom() == Some("1") {
                    Condition::Always
                } else {
                    match parse_condition(&items[0]) {
                        Some(c) => c,
                        None => continue,
                    }
                };
                let actions = items[1..].iter().filter_map(parse_action).collect();
                branches.push(CondBranch { condition, actions });
            }
            if branches.is_empty() {
                None
            } else {
                Some(Action::Cond(branches))
            }
        }
        _ => None,
    }
}

fn parse_maps(nodes: &[Sexp], spec: &mut Spec) {
    for map_node in nodes {
        let list = match map_node {
            Sexp::List(items) if !items.is_empty() => items,
            _ => continue,
        };
        let map_name = match list[0].as_atom() {
            Some(name) => name.to_string(),
            None => continue,
        };
        let mut out_entries: Vec<MapEntry> = Vec::new();

        for entry in &list[1..] {
            let entry_items = match entry {
                Sexp::List(items) if !items.is_empty() => items,
                _ => continue,
            };

            // The key can be an atom (`"k"`) or a list such as `(BackSpace)`.
            let key = match &entry_items[0] {
                Sexp::Atom(s) => s.clone(),
                Sexp::List(inner) => match inner.first().and_then(Sexp::as_atom) {
                    Some("BackSpace") => "\u{8}".to_string(),
                    Some(other) => other.to_string(),
                    None => continue,
                },
            };
            if key.is_empty() {
                continue;
            }

            let mut actions = Vec::new();
            let mut output = String::new();
            for item in &entry_items[1..] {
                if let Sexp::Atom(s) = item {
                    output = s.clone();
                    continue;
                }
                if let Some(a) = parse_action(item) {
                    actions.push(a);
                }
            }
            if !output.is_empty() {
                actions.push(Action::Insert(output));
            }
            out_entries.push(MapEntry { key, actions });
        }

        // Sort by descending key length so longest matches win.
        out_entries.sort_by(|a, b| b.key.chars().count().cmp(&a.key.chars().count()));
        spec.maps.insert(map_name, out_entries);
    }
}

fn parse_states(nodes: &[Sexp], spec: &mut Spec) {
    for state_node in nodes {
        let list = match state_node {
            Sexp::List(items) if !items.is_empty() => items,
            _ => continue,
        };
        let name = match list[0].as_atom() {
            Some(name) => name.to_string(),
            None => continue,
        };

        let mut def = StateDef::default();
        for entry in &list[1..] {
            let entry_items = match entry {
                Sexp::List(items) if !items.is_empty() => items,
                _ => continue,
            };
            let head = match entry_items[0].as_atom() {
                Some(h) => h,
                None => continue,
            };
            if head == "t" {
                for action in &entry_items[1..] {
                    if let Some(a) = parse_action(action) {
                        def.entry_actions.push(a);
                    }
                }
            } else {
                let actions = entry_items[1..].iter().filter_map(parse_action).collect();
                def.rules.push(StateRule {
                    map_name: head.to_string(),
                    actions,
                });
            }
        }
        spec.states.insert(name, def);
    }
}

/// Parse a `.mim` specification text into a [`Spec`].
pub(crate) fn parse_spec(spec_text: &str) -> Spec {
    let root = SexprParser::new(spec_text).parse_all();

    let mut spec = Spec::default();
    for node in &root {
        let list = match node {
            Sexp::List(items) if !items.is_empty() => items,
            _ => continue,
        };
        match list[0].as_atom() {
            Some("map") => parse_maps(&list[1..], &mut spec),
            Some("state") => parse_states(&list[1..], &mut spec),
            _ => {}
        }
    }
    spec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_minimal() {
        let spec = parse_spec(
            "(map (byanjon (\"k\" (set CID 1) \"ক\")))\n(state (init (byanjon (shift init))))",
        );
        let map = spec.maps.get("byanjon").unwrap();
        assert_eq!(map.len(), 1);
        assert_eq!(map[0].key, "k");
        // Two actions: the `set` and the inserted output.
        assert_eq!(map[0].actions.len(), 2);
        assert!(spec.states.contains_key("init"));
    }

    #[test]
    fn test_map_sorted_by_len() {
        let spec = parse_spec("(map (m (\"k\" \"a\") (\"kkh\" \"b\") (\"kh\" \"c\")))");
        let map = spec.maps.get("m").unwrap();
        assert_eq!(map[0].key, "kkh");
        assert_eq!(map[1].key, "kh");
        assert_eq!(map[2].key, "k");
    }
}
