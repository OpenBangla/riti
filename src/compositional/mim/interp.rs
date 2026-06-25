// m17n .mim interpreter. Ported from the khipro library (core/src/engine.cpp).
// Conversion is stateless: the whole roman "tape" is replayed through the state
// machine on every call, so backspace is just "drop last char and replay".

use std::collections::HashMap;

use super::spec::{Action, Condition, MapEntry, Spec, StateDef, StateRule};

type Vars = HashMap<String, i64>;

struct Match<'a> {
    entry: &'a MapEntry,
    rule: &'a StateRule,
    key_len: usize,
}

fn eval_cond(cond: &Condition, vars: &Vars) -> bool {
    match cond {
        Condition::Always => true,
        Condition::Equals { variable, value } => {
            vars.get(variable).copied().unwrap_or(0) == *value
        }
        Condition::And(l, r) => eval_cond(l, vars) && eval_cond(r, vars),
        Condition::Or(l, r) => eval_cond(l, vars) || eval_cond(r, vars),
    }
}

/// Resolve a `set` value: either an existing variable's value or an integer
/// literal, defaulting to `0`.
fn resolve_value(token: &str, vars: &Vars) -> i64 {
    if let Some(v) = vars.get(token) {
        return *v;
    }
    token.parse::<i64>().unwrap_or(0)
}

fn execute_actions(
    spec: &Spec,
    actions: &[Action],
    out: &mut Vec<char>,
    cursor: &mut usize,
    vars: &mut Vars,
    state_name: &mut String,
) {
    for action in actions {
        match action {
            Action::Set { name, value } => {
                let v = resolve_value(value, vars);
                vars.insert(name.clone(), v);
            }
            Action::Insert(text) => {
                let chars: Vec<char> = text.chars().collect();
                let at = (*cursor).min(out.len());
                out.splice(at..at, chars.iter().copied());
                *cursor = at + chars.len();
            }
            Action::Delete { n } => {
                if *cursor == 0 || *n <= 0 {
                    continue;
                }
                let n = *n as usize;
                let start = (*cursor).saturating_sub(n);
                if start < *cursor && start < out.len() {
                    let end = (*cursor).min(out.len());
                    out.drain(start..end);
                    *cursor = start;
                }
            }
            Action::Move { n, to_end } => {
                if *to_end {
                    *cursor = out.len();
                } else {
                    let next = *cursor as i64 + *n;
                    *cursor = next.clamp(0, out.len() as i64) as usize;
                }
            }
            Action::Shift(name) => {
                *state_name = name.clone();
                if let Some(state) = spec.states.get(name) {
                    execute_actions(spec, &state.entry_actions, out, cursor, vars, state_name);
                }
            }
            Action::Commit => {
                // No-op for output: riti manages the input session separately.
            }
            Action::Undo => {
                if *cursor > 0 && !out.is_empty() {
                    let at = *cursor - 1;
                    out.remove(at);
                    *cursor = at;
                }
            }
            Action::Cond(branches) => {
                for branch in branches {
                    if eval_cond(&branch.condition, vars) {
                        execute_actions(spec, &branch.actions, out, cursor, vars, state_name);
                        break;
                    }
                }
            }
        }
    }
}

/// Finds the longest map entry (across all rules of `state`) that matches the
/// input starting at `index`.
fn find_match<'a>(spec: &'a Spec, state: &'a StateDef, input: &[char], index: usize) -> Option<Match<'a>> {
    let mut best: Option<Match<'a>> = None;
    let mut best_len = 0;

    for rule in &state.rules {
        let entries = match spec.maps.get(&rule.map_name) {
            Some(e) => e,
            None => continue,
        };
        for entry in entries {
            let key_len = entry.key.chars().count();
            if key_len <= best_len {
                continue;
            }
            if index + key_len <= input.len() && key_matches(&entry.key, &input[index..index + key_len]) {
                best = Some(Match {
                    entry,
                    rule,
                    key_len,
                });
                best_len = key_len;
            }
        }
    }
    best
}

fn key_matches(key: &str, slice: &[char]) -> bool {
    key.chars().eq(slice.iter().copied())
}

/// Converts a roman `input` string into Bengali using the spec's state machine.
pub(crate) fn run_conversion(spec: &Spec, input: &str) -> String {
    let input: Vec<char> = input.chars().collect();
    let mut current_state = "init".to_string();
    let mut vars: Vars = HashMap::new();
    let mut out: Vec<char> = Vec::new();
    let mut cursor = 0usize;
    let mut i = 0usize;

    // Apply entry actions of the initial state.
    if let Some(state) = spec.states.get(&current_state) {
        let actions = state.entry_actions.clone();
        execute_actions(spec, &actions, &mut out, &mut cursor, &mut vars, &mut current_state);
    }

    while i < input.len() {
        let state = match spec.states.get(&current_state) {
            Some(s) => s,
            None => break,
        };

        match find_match(spec, state, &input, i) {
            None => {
                // No rule matched: pass the character through verbatim and
                // reset to the `init` state.
                let ch = input[i];
                let at = cursor.min(out.len());
                out.insert(at, ch);
                cursor = at + 1;
                i += 1;
                if current_state != "init" {
                    current_state = "init".to_string();
                    if let Some(state) = spec.states.get("init") {
                        let actions = state.entry_actions.clone();
                        execute_actions(
                            spec,
                            &actions,
                            &mut out,
                            &mut cursor,
                            &mut vars,
                            &mut current_state,
                        );
                    }
                }
            }
            Some(m) => {
                let entry_actions = m.entry.actions.clone();
                let rule_actions = m.rule.actions.clone();
                let key_len = m.key_len;
                execute_actions(
                    spec,
                    &entry_actions,
                    &mut out,
                    &mut cursor,
                    &mut vars,
                    &mut current_state,
                );
                execute_actions(
                    spec,
                    &rule_actions,
                    &mut out,
                    &mut cursor,
                    &mut vars,
                    &mut current_state,
                );
                cursor = cursor.min(out.len());
                i += key_len;
            }
        }
    }

    out.into_iter().collect()
}
