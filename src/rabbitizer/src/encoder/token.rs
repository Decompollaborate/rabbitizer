/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::str::CharIndices;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Token<'s> {
    End,
    Comma,
    Text(&'s str),
    /// a[b], a(b)
    Bracketed(&'s str, &'s str, BracketType),
}

#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub(crate) struct Tokenize<'s> {
    text: &'s str,
    char_indices: CharIndices<'s>,
    trailing_char: Option<(usize, char)>,
    trailing_token: Option<Token<'s>>,
}

impl<'s> Tokenize<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            text,
            char_indices: text.char_indices(),
            trailing_char: None,
            trailing_token: None,
        }
    }
}

impl<'s> Iterator for Tokenize<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.trailing_token.take() {
            return Some(token);
        }

        let mut current_start: Option<usize> = None;
        let mut bracket_start: Option<(usize, char)> = None;

        let mut iterator = self
            .trailing_char
            .take()
            .into_iter()
            .chain(self.char_indices.by_ref());
        while let Some((index, c)) = iterator.next() {
            if matches!(c, '\n' | ';') {
                return match current_start {
                    None => Some(Token::End),
                    Some(i) => {
                        self.trailing_token = Some(Token::End);
                        Some(Token::Text(&self.text[i..index]))
                    }
                };
            }

            if matches!(c, ',') {
                return match current_start {
                    None => Some(Token::Comma),
                    Some(i) => {
                        self.trailing_token = Some(Token::Comma);
                        Some(Token::Text(&self.text[i..index]))
                    }
                };
            }

            if matches!(c, ')' | ']') && bracket_start.is_some() {
                let (bracket_index, bracket_char) = bracket_start.unwrap();
                let parethesis = bracket_char == '(' && c == ')';
                let brackets = bracket_char == '[' && c == ']';
                if parethesis || brackets {
                    let i = current_start.unwrap();
                    let left = &self.text[i..bracket_index];
                    let right = &self.text[bracket_index + 1..index];
                    if parethesis {
                        return Some(Token::Bracketed(
                            left.trim(),
                            right.trim(),
                            BracketType::Parenthesis,
                        ));
                    } else if brackets {
                        return Some(Token::Bracketed(
                            left.trim(),
                            right.trim(),
                            BracketType::Brackets,
                        ));
                    } else {
                        unreachable!()
                    }
                }
            }

            if c.is_whitespace() {
                match current_start {
                    None => {} // keep looking
                    Some(i) => {
                        // Do not yield at the very first space if we are
                        // inside a bracket-like expression.
                        if bracket_start.is_none() {
                            let mut yield_value = true;
                            // Check if after this token there's a bracket start character.
                            if let Some((i2, c2)) = iterator.find(|&(_, x)| !x.is_whitespace()) {
                                if matches!(c2, '(' | '[') {
                                    bracket_start.get_or_insert((i2, c2));
                                    yield_value = false;
                                } else {
                                    // Ensure we don't lose this value.
                                    self.trailing_char = Some((i2, c2));
                                }
                            }

                            if yield_value {
                                return Some(Token::Text(&self.text[i..index]));
                            }
                        }
                    }
                }
            } else {
                let already_started = current_start.is_some();
                // Replace if empty
                current_start.get_or_insert(index);

                if already_started && matches!(c, '(' | '[') {
                    bracket_start.get_or_insert((index, c));
                }
            }
        }

        match current_start {
            None => None,
            Some(i) => Some(Token::Text(&self.text[i..])),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum BracketType {
    Brackets,
    Parenthesis,
}

impl BracketType {
    #[must_use]
    pub const fn left(self) -> char {
        match self {
            Self::Brackets => '[',
            Self::Parenthesis => '(',
        }
    }
    #[must_use]
    pub const fn right(self) -> char {
        match self {
            Self::Brackets => ']',
            Self::Parenthesis => ')',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_addiu() {
        let s = "addiu       $sp, $sp, -0x740";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(tokenizer.next(), Some(Token::Text("addiu")));
        assert_eq!(tokenizer.next(), Some(Token::Text("$sp")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("$sp")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("-0x740")));
        assert_eq!(tokenizer.next(), None)
    }

    #[test]
    fn test_tokenizer_parethesised() {
        // 0xAC24E190
        static CASES: [&str; 8] = [
            "sw          $a0, -0x1E70($at)",
            "sw          $a0, -0x1E70   ($at)",
            "sw          $a0, -0x1E70( $at)",
            "sw          $a0, -0x1E70($at )",
            "sw          $a0, -0x1E70( $at )",
            "sw          $a0, -0x1E70  ( $at)",
            "sw          $a0, -0x1E70  ($at )",
            "sw          $a0, -0x1E70  ( $at )",
        ];
        for s in &CASES {
            let mut tokenizer = Tokenize::new(s);

            assert_eq!(tokenizer.next(), Some(Token::Text("sw")));
            assert_eq!(tokenizer.next(), Some(Token::Text("$a0")));
            assert_eq!(tokenizer.next(), Some(Token::Comma));
            assert_eq!(
                tokenizer.next(),
                Some(Token::Bracketed("-0x1E70", "$at", BracketType::Parenthesis))
            );
            assert_eq!(tokenizer.next(), None)
        }
    }
}
