use std::collections::HashMap;
use std::iter;

use rand::seq::SliceRandom;

pub struct Brain {
    words: HashMap<String, HashMap<String, usize>>,
}

impl Default for Brain {
    fn default() -> Self {
        Self {
            words: Default::default(),
        }
    }
}

impl Brain {
    pub fn train(&mut self, text: &str) {
        let mut prev_word = None;

        for word in Self::tokenize(text) {
            if let Some(prev_word) = prev_word.replace(word) {
                *self
                    .words
                    .entry(prev_word.to_string())
                    .or_default()
                    .entry(word.to_string())
                    .or_default() += 1;
            }
        }
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = Self::tokenize(prompt)
            .map(|word| word.to_string())
            .collect();

        let mut rng = rand::thread_rng();

        while out.len() < length {
            let last_word = out.last().unwrap();

            if let Some(next_words) = self.words.get(last_word) {
                let next_words: Vec<_> = next_words.iter().collect();

                let next_word = next_words
                    .choose_weighted(&mut rng, |(_word, frequency)| *frequency)
                    .unwrap();
                out.push(next_word.0.to_string());
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
