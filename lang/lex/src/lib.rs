use crate::lexer::Lexer;
use crate::rule::LexingRule;
use crate::token::{Token, TokenType, TokenType::*};

pub mod token;

mod pos;
mod rule;
mod lexer;

#[cfg(test)]
mod tests;

pub fn tokenize(content: &str) -> Vec<Token> {
    let lexer = Lexer::new(build_rules());
    lexer.tokenize(content)
}

fn build_rules() -> Vec<LexingRule> {
    let mut rules = Vec::new();

    macro_rules! add_regex_pattern {
        ($typ:expr, $pattern:expr) => {
            {
                let rule = find_lexing_rule(&mut rules, $typ);
                rule.add_escaped_regex_pattern($pattern.to_string());
            }
        };
    }

    { // Whitespace
        let rule = find_lexing_rule(&mut rules, Whitespace);

        rule.add_regex_pattern("[ \t\r\n]+".to_string());
        rule.add_regex_pattern("//[^\r\n]".to_string());

        rule.add_delimited_escaped_regex_pattern("/*".to_string(), String::new(), "*/".to_string());
    }

    { // Name
        let rule = find_lexing_rule(&mut rules, Name);

        rule.add_regex_pattern("[a-zA-Z][a-zA-Z0-9_/]*".to_string());
    }

    { // Number
        let rule = find_lexing_rule(&mut rules, Number);

        rule.add_regex_pattern("[0-9_]*[\\.[0-9_]]?".to_string());
    }

    add_regex_pattern!(OpenBrace, "(");
    add_regex_pattern!(CloseBrace, ")");

    add_regex_pattern!(OpenSquareBrace, "[");
    add_regex_pattern!(CloseSquareBrace, "]");

    add_regex_pattern!(OpenCurlyBrace, "{");
    add_regex_pattern!(CloseCurlyBrace, "}");

    add_regex_pattern!(Equals, "=");

    add_regex_pattern!(Pipe, "|");

    add_regex_pattern!(Greater, ">");

    add_regex_pattern!(Less, "<");

    add_regex_pattern!(Exclamation, "!");

    add_regex_pattern!(Question, "?");

    add_regex_pattern!(At, "@");

    add_regex_pattern!(Dollar, "$");

    add_regex_pattern!(And, "&");

    add_regex_pattern!(Percent, "%");

    add_regex_pattern!(Slash, "/");

    add_regex_pattern!(Backslash, "\\");

    add_regex_pattern!(Colon, ":");

    add_regex_pattern!(Semicolon, ";");

    add_regex_pattern!(Plus, "+");

    add_regex_pattern!(Minus, "-");

    add_regex_pattern!(Star, "*");

    add_regex_pattern!(Dot, ".");

    add_regex_pattern!(Comma, ",");

    add_regex_pattern!(Hashtag, "#");

    add_regex_pattern!(Tilde, "~");

    add_regex_pattern!(Apostrophe, "'");

    add_regex_pattern!(Caret, "^");

    add_regex_pattern!(Degree, "°");

    add_regex_pattern!(QuotationMark, "\"");

    add_regex_pattern!(Underscore, "_");

    add_regex_pattern!(Backtick, "`");

    add_regex_pattern!(Prime, "´");

    rules
}

pub fn find_lexing_rule(rules: &mut Vec<LexingRule>, token_type: TokenType) -> &mut LexingRule {
    let mut found_rule = None;

    for rule in &mut *rules {
        if rule.token_type().eq(&token_type) {
            found_rule = Some(rule);
        }
    }

    rules.push(LexingRule::new(Vec::new(), token_type));

    find_lexing_rule(rules, token_type)
}

