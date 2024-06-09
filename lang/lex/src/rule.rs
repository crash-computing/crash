use regex::{Regex, RegexBuilder};
use crate::token::TokenType;

#[derive(Clone)]
pub struct LexingRule {
    patterns: Vec<Regex>,
    t_type: TokenType,
}

impl LexingRule {
    pub const fn new(patterns: Vec<Regex>, t_type: TokenType) -> Self {
        Self { patterns, t_type }
    }

    pub fn add_escaped_regex_pattern(&mut self, pattern: String) {
        let escaped_pattern = regex_escape(pattern);
        self.add_regex_pattern(escaped_pattern)
    }

    pub fn add_regex_pattern(&mut self, pattern: String) {
        match Regex::new(&pattern) {
            Ok(regex) => self.patterns.push(regex),
            Err(err) => {
                panic!("Cannot create regex of pattern '{pattern}': {err}")
            }
        }
    }

    pub fn add_delimited_escaped_regex_pattern(&mut self, opening: String, escape: String, closing: String) {
        let escaped_opening = regex_escape(opening.clone());
        let escaped_escape = regex_escape(escape.clone());
        let escaped_closing = regex_escape(closing.clone());

        let regex_pattern;

        if escape.is_empty() {
            regex_pattern = format!("{escaped_opening}.*?{escaped_closing}");
        } else {
            regex_pattern = format!("{escaped_opening}(?:{escaped_escape}(?:{escaped_escape}|{escaped_closing}|(?!{escaped_closing}).)|(?!{escaped_escape}|{escaped_closing}).)*{escaped_closing}");
        }

        let regex_builder = RegexBuilder::new(&regex_pattern)
            .dot_matches_new_line(true)
            .build();

        match regex_builder {
            Ok(regex) => self.patterns.push(regex),
            Err(err) => {
                panic!("Cannot create delimited regex pattern '{regex_pattern}': {err}");
            }
        }
    }

    pub fn patterns(&self) -> &Vec<Regex> {
        &self.patterns
    }

    pub fn token_type(&self) -> &TokenType {
        &self.t_type
    }
}

fn regex_escape(input: String) -> String {
    let mut builder = String::new();
    for char in input.chars() {
        match char {
            '\0' | '\n' | '\r' | '\t' | '\\' | '^' | '$' | '?' | '|' | '*' | '/' | '+' | '.' | '(' | ')' | '[' | ']' | '{' | '}' => {
                builder.push('\\');
                builder.push(char);
                continue;
            }
            _ => {}
        }

        let digit = char as u32;

        if digit > 0xff {
            builder.push_str("\\u");
            builder.push_str(format!("{}04x", char).as_str());
        } else if is_iso_control(digit) {
            builder.push_str("\\x");
            builder.push_str(format!("{}2x", char).as_str());
        } else {
            builder.push(char);
        }
    }

    builder
}

fn is_iso_control(digit: u32) -> bool {
    return digit <= 0x9F && (digit >= 0x7F || (0 == (digit >> 5)));
}