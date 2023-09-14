use rand::seq::SliceRandom;

pub struct Brain {
    words: Vec<String>,
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
        self.words = text.split(' ').map(|word| word.to_string()).collect()
    }

    pub fn prompt(&self, prompt: &str, length: usize) -> String {
        let mut out: Vec<_> = prompt.split(' ').map(|word| word.to_string()).collect();

        let mut rng = rand::thread_rng();

        while out.len() < length {
            out.push(self.words.choose(&mut rng).unwrap().clone());
        }

        out.join(" ")
    }
}
