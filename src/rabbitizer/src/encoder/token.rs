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
    /// a\[b\], a(b)
    Bracketed(&'s str, &'s str, BracketType),
    /// \[b\], (b)
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
    trailing_token: Option<(Token<'s>, usize, usize)>,
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

    pub fn get_text_range(&self, start: usize, end: usize) -> &'s str {
        if end > self.text.len() {
            &self.text[start..]
        } else {
            &self.text[start..end]
        }
    }

    fn decide_text_to_return(
        &mut self,
        current_start: Option<usize>,
        dot_index: Option<usize>,
        index: usize,
        trailing_token: Option<(Token<'s>, usize, usize)>,
    ) -> Option<(Token<'s>, usize, usize)> {
        match (current_start, dot_index) {
            (None, _) => trailing_token,
            (Some(i), Some(dot)) if index > dot + 1 => {
                // `if index > dot + 1` checks we don't have a trailing dot.
                self.trailing_token = trailing_token;
                Some((
                    Token::DottedText(TokenDottedText::new(
                        &self.text[i..index],
                        &self.text[i..dot],
                        &self.text[dot..index],
                    )),
                    i,
                    index,
                ))
            }
            (Some(i), _) => {
                self.trailing_token = trailing_token;
                Some((Token::Text(&self.text[i..index]), i, index))
            }
        }
    }

    fn decide_bracket_to_return(
        text: &'s str,
        iterator: &mut impl Iterator<Item = (usize, char)>,
        current_start: Option<usize>,
        (bracket_index, bracket_start): (usize, char),
    ) -> Option<(Token<'s>, usize, usize)> {
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
        let (token, start_pos) = match current_start {
            None => (
                Token::BracketSolo(bracketed_part, bracket_type),
                bracket_index,
            ),
            Some(i) => {
                let left = &text[i..bracket_index];

                (
                    Token::Bracketed(left.trim(), bracketed_part.trim(), bracket_type),
                    i,
                )
            }
        };

        Some((token, start_pos, end_pos + 1))
    }
}

impl<'s> Iterator for Tokenize<'s> {
    type Item = (Token<'s>, usize, usize);

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
                    Some((Token::End, index, index + 1)),
                );
            }

            if matches!(c, ',') {
                return self.decide_text_to_return(
                    current_start,
                    dot_index,
                    index,
                    Some((Token::Comma, index, index + 1)),
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

    use pretty_assertions::assert_eq;

    fn test_tokenizer(s: &str, mut tokenizer: Tokenize, test_data: &[(Token, usize, usize)]) {
        for (expected_token, expected_start, expected_end) in test_data {
            let (token, start, end) = tokenizer.next().unwrap();
            assert_eq!(token, *expected_token);
            assert_eq!(start, *expected_start);
            assert_eq!(end, *expected_end);

            match token {
                Token::End => {}
                Token::Comma => assert_eq!(&s[start..end], ","),
                Token::Text(text) => assert_eq!(&s[start..end], text),
                Token::Bracketed(left, right, _bracket_type) => {
                    assert!(&s[start..end].starts_with(left));
                    assert!(&s[start..end].contains(right));
                }
                Token::BracketSolo(text, _bracket_type) => assert!(&s[start..end].contains(text)),
                Token::DottedText(TokenDottedText { full, .. }) => assert_eq!(&s[start..end], full),
            }
        }

        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_addiu() {
        static TEST_DATA: [(Token, usize, usize); 6] = [
            (Token::Text("addiu"), 0, 5),
            (Token::Text("$sp"), 12, 15),
            (Token::Comma, 15, 16),
            (Token::Text("$sp"), 17, 20),
            (Token::Comma, 20, 21),
            (Token::Text("-0x740"), 22, 28),
        ];

        let s = "addiu       $sp, $sp, -0x740";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
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

            assert_eq!(tokenizer.next().map(|x| x.0), Some(Token::Text("sw")));
            assert_eq!(tokenizer.next().map(|x| x.0), Some(Token::Text("$a0")));
            assert_eq!(tokenizer.next().map(|x| x.0), Some(Token::Comma));
            assert_eq!(
                tokenizer.next().map(|x| x.0),
                Some(Token::Bracketed("-0x1E70", "$at", BracketType::Parenthesis))
            );
            assert_eq!(tokenizer.next().map(|x| x.0), None);
        }
    }

    #[test]
    fn test_tokenizer_parethesised_single() {
        static TEST_DATA: [(Token, usize, usize); 4] = [
            (Token::Text("sw"), 0, 2),
            (Token::Text("$a0"), 12, 15),
            (Token::Comma, 17, 18),
            (
                Token::Bracketed("-0x1E70", "$at", BracketType::Parenthesis),
                21,
                37,
            ),
        ];

        let s = "sw          $a0  ,   -0x1E70  ( $at )";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_multiline() {
        static TEST_DATA: [(Token, usize, usize); 11] = [
            (Token::Text("lui"), 0, 3),
            (Token::Text("$v0"), 4, 7),
            (Token::Comma, 7, 8),
            (Token::Text("0x8020"), 9, 15),
            (Token::End, 16, 17),
            (Token::Text("addiu"), 18, 23),
            (Token::Text("$v0"), 24, 27),
            (Token::Comma, 27, 28),
            (Token::Text("$v0"), 29, 32),
            (Token::Comma, 32, 33),
            (Token::Text("0x1234"), 34, 40),
        ];

        let s = "lui $v0, 0x8020 \n addiu $v0, $v0, 0x1234";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_bracket_solo() {
        static TEST_DATA: [(Token, usize, usize); 6] = [
            (
                Token::DottedText(TokenDottedText::new("vrot.q", "vrot", ".q")),
                0,
                6,
            ),
            (Token::Text("C002"), 12, 16),
            (Token::Comma, 16, 17),
            (Token::Text("S400"), 18, 22),
            (Token::Comma, 22, 23),
            (Token::BracketSolo("C,S,S,S", BracketType::Brackets), 24, 33),
        ];

        let s = "vrot.q      C002, S400, [C,S,S,S]";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_dotted_text() {
        static TEST_DATA: [(Token, usize, usize); 6] = [
            (
                Token::DottedText(TokenDottedText::new("vadda.xyz", "vadda", ".xyz")),
                0,
                9,
            ),
            (Token::Text("ACC"), 12, 15),
            (Token::Comma, 15, 16),
            (Token::Text("$vf0"), 17, 21),
            (Token::Comma, 21, 22),
            (Token::Text("$vf7"), 23, 27),
        ];

        let s = "vadda.xyz   ACC, $vf0, $vf7";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_double_dotted_text() {
        static TEST_DATA: [(Token, usize, usize); 4] = [
            (
                Token::DottedText(TokenDottedText::new("round.l.s", "round", ".l.s")),
                0,
                9,
            ),
            (Token::Text("$f0"), 12, 15),
            (Token::Comma, 15, 16),
            (Token::Text("$f2"), 17, 20),
        ];

        let s = "round.l.s   $f0, $f2";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_trailing_dot() {
        static TEST_DATA: [(Token, usize, usize); 1] = [(Token::Text("asdf."), 0, 5)];

        let s = "asdf.";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_branch_absolute() {
        static TEST_DATA: [(Token, usize, usize); 2] =
            [(Token::Text("b"), 0, 1), (Token::Text("0x80000014"), 8, 18)];

        let s = "b       0x80000014";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_branch_computed() {
        static TEST_DATA: [(Token, usize, usize); 4] = [
            (Token::Text("b"), 0, 1),
            (Token::Text("."), 8, 9),
            (Token::Text("+"), 10, 11),
            (Token::Text("0xC"), 12, 15),
        ];

        let s = "b       . + 0xC";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }

    #[test]
    fn test_tokenizer_branch_full_expression() {
        static TEST_DATA: [(Token, usize, usize); 5] = [
            (Token::Text("b"), 0, 1),
            (Token::Text("."), 8, 9),
            (Token::Text("+"), 10, 11),
            (Token::Text("4"), 12, 13),
            (
                Token::Bracketed("+", "0x2 << 2", BracketType::Parenthesis),
                14,
                26,
            ),
        ];

        let s = "b       . + 4 + (0x2 << 2)";
        let tokenizer = Tokenize::new(s);

        test_tokenizer(s, tokenizer, &TEST_DATA);
    }
}
