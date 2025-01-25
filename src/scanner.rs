use crate::tokens::Token;

#[derive(Default)]
pub struct Scanner {
    tokens: Vec<Token>,
    source: String,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    fn scan(&self) {
        for line in self.source {}
    }
}
