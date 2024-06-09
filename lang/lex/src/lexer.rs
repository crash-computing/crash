use crate::pos::TokenPosition;
use crate::rule::LexingRule;
use crate::token::{Position, Token};

pub struct Lexer {
    rules: Vec<LexingRule>
}

impl Lexer {

    pub fn new(rules: Vec<LexingRule>) -> Self {
        Self { rules }
    }

    pub fn tokenize(&self, mut input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut current_column = 1;
        let mut current_line = 1;

        while !input.is_empty() {
            let mut matched_length = 0;
            let mut matched_type = None;

            for rule in self.rules.iter() {
                for pattern in rule.patterns() {

                    let Some(matcher) = pattern.find(&input) else {
                        continue
                    };

                    if matcher.start() != 0 {
                        continue
                    }

                    let length = matcher.end();

                    if length > matched_length {
                        matched_length = length;
                        matched_type = Some(rule.token_type());
                    }
                }
            }

            if matched_length > 0 {
                let substring = &input[..matched_length];

                let token = Token::new(
                    *matched_type.expect("Expected token"),
                    substring.to_string(),
                    TokenPosition::new(
                        Position::new(current_line, current_column),
                        Position::new(current_line, substring.len() as u32 + current_column)
                    )
                );

                tokens.push(token);

                for c in substring.chars() {
                    if c == '\n' {
                        current_line += 1;
                        current_column +=1;
                        continue
                    }
                    current_column += 1;
                }

                input = &input[matched_length..];
                continue
            }

            panic!("Unknown token at {current_line}:{current_column}")
        }

        tokens.push(Token::eof());

        tokens
    }
}