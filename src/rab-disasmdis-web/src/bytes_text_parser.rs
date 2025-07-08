/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use core::str::CharIndices;

pub struct BytesTextParser<'s> {
    char_indices: CharIndices<'s>,
}

impl<'s> BytesTextParser<'s> {
    pub fn new(text: &'s str) -> Self {
        Self {
            char_indices: text.char_indices(),
        }
    }
}

impl Iterator for BytesTextParser<'_> {
    type Item = ParsedTextResult;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer: [u8; 4] = [0; 4];
        let mut buffer_index = 0;
        for (index, c) in self.char_indices.by_ref() {
            match c {
                '0'..='9' => {
                    let value = c as u8 - b'0';
                    let i = buffer_index / 2;
                    let nibble = buffer_index % 2 != 0;

                    if nibble {
                        buffer[i] |= value;
                    } else {
                        buffer[i] = value << 4;
                    }
                    buffer_index += 1;
                }
                'A'..='F' => {
                    let value = c as u8 - b'A' + 10;
                    let i = buffer_index / 2;
                    let nibble = buffer_index % 2 != 0;

                    if nibble {
                        buffer[i] |= value;
                    } else {
                        buffer[i] = value << 4;
                    }
                    buffer_index += 1;
                }
                'a'..='f' => {
                    let value = c as u8 - b'a' + 10;
                    let i = buffer_index / 2;
                    let nibble = buffer_index % 2 != 0;

                    if nibble {
                        buffer[i] |= value;
                    } else {
                        buffer[i] = value << 4;
                    }
                    buffer_index += 1;
                }
                'x' | 'X' => buffer_index = buffer_index.saturating_sub(1),
                ':' | '-' | '+' | ',' => {}  // ignore those
                _ if c.is_whitespace() => {} // ignore whitespace
                _ => return Some(ParsedTextResult::InvalidCharacter(c, index)),
            }

            if buffer_index >= 8 {
                return Some(ParsedTextResult::Bytes(buffer));
            }
        }

        // todo!()
        None
    }
}

pub enum ParsedTextResult {
    Bytes([u8; 4]),
    InvalidCharacter(char, usize),
}
