/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::iter::Peekable;
use core::str::CharIndices;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Token<'s> {
    End,
    Comma,
    Text(&'s str),
}

#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub(crate) struct Tokenize<'s> {
    text: &'s str,
    char_indices: Peekable<CharIndices<'s>>,
}

impl<'s> Tokenize<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            text,
            char_indices: text.char_indices().peekable(),
        }
    }
}

impl<'s> Iterator for Tokenize<'s> {
    type Item = Token<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_start: Option<usize> = None;

        while let Some(&(index, c)) = self.char_indices.peek() {
            if matches!(c, '\n' | ';') {
                return match current_start {
                    None => {
                        // Advance the iterator
                        self.char_indices.next();
                        Some(Token::End)
                    }
                    Some(i) => Some(Token::Text(&self.text[i..index])),
                };
            }

            if matches!(c, ',') {
                return match current_start {
                    None => {
                        // Advance the iterator
                        self.char_indices.next();
                        Some(Token::Comma)
                    }
                    Some(i) => Some(Token::Text(&self.text[i..index])),
                };
            }

            if c.is_whitespace() {
                match current_start {
                    None => {} // keep looking
                    Some(i) => {
                        // Advance the iterator
                        self.char_indices.next();
                        return Some(Token::Text(&self.text[i..index]));
                    }
                }
            } else {
                // Replace if empty
                current_start.get_or_insert(index);
            }

            // Advance the iterator
            self.char_indices.next();
        }

        match current_start {
            None => None,
            Some(i) => Some(Token::Text(&self.text[i..])),
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
}
