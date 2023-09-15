use std::collections::HashMap;
use std::iter;

use rand::seq::SliceRandom;

pub struct Brain {
    tokens: HashMap<Vec<String>, HashMap<String, usize>>,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
        }
    }
}

impl Brain {
    const MAX_CONTEXT_SIZE: usize = 5;

    pub fn train(&mut self, text: &str) {
        let mut context: Vec<&str> = Vec::new();

        for token in Self::tokenize(text) {
            for cs in 1..=context.len() {
                let context = context[(context.len() - cs)..context.len()]
                    .iter()
                    .map(|token| token.to_string())
                    .collect();
                *self
                    .tokens
                    .entry(context)
                    .or_default()
                    .entry(token.to_string())
                    .or_default() += 1;
            }

            context.push(token);

            if context.len() > Self::MAX_CONTEXT_SIZE {
                context.remove(0);
            }
        }
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = Self::tokenize(prompt).collect();

        let mut rng = rand::thread_rng();

        while out.len() < length {
            let mut next_token = None;

            for cs in (1..=Self::MAX_CONTEXT_SIZE).rev() {
                if cs > out.len() {
                    continue;
                }
                let context: Vec<_> = out[(out.len() - cs)..out.len()]
                    .iter()
                    .map(|token| token.to_string())
                    .collect();

                if let Some(next_tokens) = self.tokens.get(&context) {
                    let next_tokens: Vec<_> = next_tokens.iter().collect();

                    next_token = Some(
                        next_tokens
                            .choose_weighted(&mut rng, |(_token, frequency)| *frequency)
                            .unwrap()
                            .0,
                    );

                    break;
                }
            }

            if let Some(next_token) = next_token {
                out.push(next_token);
            } else {
                break;
            }
        }
        out.join("")
    }

    pub fn tokenize(s: &str) -> impl Iterator<Item = &str> {
        let mut chars = s.char_indices().peekable();

        iter::from_fn(move || loop {
            let (idx, ch) = chars.next()?;

            if ch.is_alphanumeric() {
                let idx_from = idx;
                let mut idx_to = idx + ch.len_utf8();

                while let Some((idx, ch)) = chars.peek() {
                    if ch.is_ascii_alphanumeric() {
                        idx_to = idx + ch.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                return Some(&s[idx_from..idx_to]);
            } else {
                let idx_from = idx;
                let idx_to = idx + ch.len_utf8();

                return Some(&s[idx_from..idx_to]);
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use test_case::test_case;

    #[test_case(
        "Hello, World!",
        &["Hello",",", " ","World", "!"]
    )]
    #[test_case(
        "#include <coffee.h>",
        &["#","include"," ","<","coffee",".","h",">"]
    )]
    #[test_case(
        "123 + 234 = 0xCAFEBABE",
        &["123"," ","+"," ","234"," ","="," ","0xCAFEBABE"]
    )]
    fn tokenize(given: &str, expected: &[&str]) {
        let actual: Vec<_> = Brain::tokenize(given).collect();

        // let expected: Vec<_> = expected.iter()
        //     .map(|token| token.to_string())
        //     .collect();
        assert_eq!(actual.as_slice(), expected);
    }
}
