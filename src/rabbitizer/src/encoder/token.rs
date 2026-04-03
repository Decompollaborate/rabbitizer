/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct TokenDottedText<'s> {
    pub(crate) full: &'s str,
    pub(crate) left: &'s str,
    pub(crate) dotted: &'s str,
}

impl<'s> TokenDottedText<'s> {
    pub(crate) const fn new(full: &'s str, text: &'s str, dotted: &'s str) -> Self {
        Self {
            full,
            left: text,
            dotted,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum Token<'s> {
    End,
    Comma,
    Text(&'s str),
    /// a[b], a(b)
    Bracketed(&'s str, &'s str, BracketType),
    /// [b], (b)
    BracketSolo(&'s str, BracketType),
    /// a.b
    /// `a` and `b` are be non-empty (doesn't support `.b`, `a.`).
    /// Those cases return `Text`.
    ///
    /// `a` has 0 dots.
    /// `b` has at least 1 dot, maybe more.
    DottedText(TokenDottedText<'s>),
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

    fn decide_text_to_return(
        &mut self,
        current_start: Option<usize>,
        dot_index: Option<usize>,
        index: usize,
        trailing_token: Option<Token<'s>>,
    ) -> Option<Token<'s>> {
        match (current_start, dot_index) {
            (None, _) => trailing_token,
            (Some(i), Some(dot)) if index > dot + 1 => {
                // `if index > dot + 1` checks we don't have a trailing dot.
                self.trailing_token = trailing_token;
                Some(Token::DottedText(TokenDottedText::new(
                    &self.text[i..index],
                    &self.text[i..dot],
                    &self.text[dot..index],
                )))
            }
            (Some(i), _) => {
                self.trailing_token = trailing_token;
                Some(Token::Text(&self.text[i..index]))
            }
        }
    }

    fn decide_bracket_to_return(
        text: &'s str,
        iterator: &mut impl Iterator<Item = (usize, char)>,
        current_start: Option<usize>,
        (bracket_index, bracket_start): (usize, char),
    ) -> Option<Token<'s>> {
        let bracket_type = if bracket_start == '(' {
            BracketType::Parenthesis
        } else if bracket_start == '[' {
            BracketType::Brackets
        } else {
            return None;
        };
        let bracket_end = bracket_type.right();

        // Consume the iterator until we find the first closing bracket
        let (end_pos, _) = iterator.find(|&(_other_i, other_c)| other_c == bracket_end)?;

        let bracketed_part = &text[bracket_index + 1..end_pos];
        let token = match current_start {
            None => Token::BracketSolo(bracketed_part, bracket_type),
            Some(i) => {
                let left = &text[i..bracket_index];

                Token::Bracketed(left.trim(), bracketed_part.trim(), bracket_type)
            }
        };

        Some(token)
    }
}

impl<'s> Iterator for Tokenize<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.trailing_token.take() {
            return Some(token);
        }

        let mut current_start: Option<usize> = None;
        let mut bracket_start_info: Option<(usize, char)> = None;
        let mut dot_index: Option<usize> = None;

        let mut iterator = self
            .trailing_char
            .take()
            .into_iter()
            .chain(self.char_indices.by_ref());
        while let Some((index, c)) = iterator.next() {
            if matches!(c, '/') && self.text[index + 1..].starts_with('*') {
                // Found multiline coment.
                // Skip over it.
                iterator.find(|&(other_i, other_c)| {
                    other_c == '*' && self.text[other_i + 1..].starts_with('/')
                });
                // Skip the trailing `/` from the `*/`.
                iterator.next();
                continue;
            }
            if matches!(c, '#') {
                // Single line comment.
                // Skip everything until newline
                iterator.find(|&(_other_i, other_c)| other_c == '\n');
                continue;
            }

            if matches!(c, '\n' | ';') {
                return self.decide_text_to_return(
                    current_start,
                    dot_index,
                    index,
                    Some(Token::End),
                );
            }

            if matches!(c, ',') {
                return self.decide_text_to_return(
                    current_start,
                    dot_index,
                    index,
                    Some(Token::Comma),
                );
            }

            if matches!(c, '(' | '[') || bracket_start_info.is_some() {
                return Self::decide_bracket_to_return(
                    self.text,
                    &mut iterator,
                    current_start,
                    bracket_start_info.unwrap_or((index, c)),
                );
            }

            if matches!(c, '.') && dot_index.is_none() && current_start.is_some() {
                // Track the first dot we see.
                // Only track it if we have seen something that isn't a dot before.
                dot_index = Some(index);
                continue;
            }

            if c.is_whitespace() {
                match current_start {
                    None => {} // keep looking
                    Some(i) => {
                        // Do not yield at the very first space if we are
                        // inside a bracket-like expression.
                        if bracket_start_info.is_none() {
                            let mut yield_value = true;
                            // Check if after this token there's a bracket start character.
                            if let Some((i2, c2)) =
                                iterator.find(|&(_, x)| !x.is_whitespace() || x == '\n')
                            {
                                if matches!(c2, '(' | '[') {
                                    bracket_start_info.get_or_insert((i2, c2));
                                    yield_value = false;
                                } else {
                                    // Ensure we don't lose this value.
                                    self.trailing_char = Some((i2, c2));
                                }
                            }

                            if yield_value {
                                return self.decide_text_to_return(Some(i), dot_index, index, None);
                            }
                        }
                    }
                }
            } else {
                // Replace if empty
                current_start.get_or_insert(index);
            }
        }

        self.decide_text_to_return(current_start, dot_index, self.text.len(), None)
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
        assert_eq!(tokenizer.next(), None);
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
            assert_eq!(tokenizer.next(), None);
        }
    }

    #[test]
    fn test_tokenizer_multiline() {
        let s = "lui $v0, 0x8020 \n addiu $v0, $v0, 0x1234";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(tokenizer.next(), Some(Token::Text("lui")));
        assert_eq!(tokenizer.next(), Some(Token::Text("$v0")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("0x8020")));
        assert_eq!(tokenizer.next(), Some(Token::End));
        assert_eq!(tokenizer.next(), Some(Token::Text("addiu")));
        assert_eq!(tokenizer.next(), Some(Token::Text("$v0")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("$v0")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("0x1234")));

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_bracket_solo() {
        let s = "vrot.q      C002, S400, [C,S,S,S]";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(
            tokenizer.next(),
            Some(Token::DottedText(TokenDottedText::new(
                "vrot.q", "vrot", ".q"
            )))
        );
        assert_eq!(tokenizer.next(), Some(Token::Text("C002")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("S400")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(
            tokenizer.next(),
            Some(Token::BracketSolo("C,S,S,S", BracketType::Brackets))
        );
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_dotted_text() {
        let s = "vadda.xyz   ACC, $vf0, $vf7";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(
            tokenizer.next(),
            Some(Token::DottedText(TokenDottedText::new(
                "vadda.xyz",
                "vadda",
                ".xyz"
            )))
        );
        assert_eq!(tokenizer.next(), Some(Token::Text("ACC")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("$vf0")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("$vf7")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_double_dotted_text() {
        let s = "round.l.s   $f0, $f2";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(
            tokenizer.next(),
            Some(Token::DottedText(TokenDottedText::new(
                "round.l.s",
                "round",
                ".l.s"
            )))
        );
        assert_eq!(tokenizer.next(), Some(Token::Text("$f0")));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Text("$f2")));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_trailing_dot() {
        let s = "asdf.";
        let mut tokenizer = Tokenize::new(s);

        assert_eq!(tokenizer.next(), Some(Token::Text("asdf.")));
        assert_eq!(tokenizer.next(), None);
    }
}
