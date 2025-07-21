/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::encoded_field_mask::EncodedFieldMask;
use crate::encoder::token::{BracketType, Token, Tokenize};
use crate::encoder::EncodingError;
use crate::opcodes::Opcode;
use crate::operands::Operand;
use crate::registers::*;
#[cfg(feature = "R4000ALLEGREX")]
use crate::registers_meta::R4000AllegrexVectorRegister;
use crate::registers_meta::Register;
use crate::utils::{self, DoubleOptIterator};

impl Operand {
    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn encode_to_bits<'s>(
        self,
        token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
        abi: Abi,
        allow_dollarless: bool,
        opcode: Opcode,
    ) -> Result<EncodedOperandBits, EncodingError<'s>> {
        let Some((token, mut next_token)) = token_stream.next() else {
            return Err(EncodingError::RanOutOfTokens(opcode, self));
        };

        let val = match self {
            Self::ALL_EMPTY => None,
            Self::core_rs | Self::core_rt | Self::core_rd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Gpr>(text, abi, allow_dollarless)
            }
            Self::core_sa => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            Self::core_zero => None /*Self::core_zero()*/,
            Self::core_cop0d => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop0>(text, abi, allow_dollarless)
            }
            Self::core_cop0cd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop0Control>(text, abi, allow_dollarless)
            }
            Self::core_fs | Self::core_ft | Self::core_fd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop1>(text, abi, allow_dollarless)
            }
            Self::core_cop1cs => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop1Control>(text, abi, allow_dollarless)
            }
            Self::core_cop2t | Self::core_cop2d => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop2>(text, abi, allow_dollarless)
            }
            Self::core_cop2cd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<Cop2Control>(text, abi, allow_dollarless)
            }

            Self::core_op => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            Self::core_hint => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }

            Self::core_code => {
                let text = operand_text_from_token(token, opcode, self)?;
                let parsed = match utils::u16_hex_from_str(text).ok() {
                    None => None,
                    Some(code_upper) => {
                        if next_token == Some(Token::Comma) {
                            let code_lower_text = request_next_text(
                                token_stream,
                                &mut next_token,
                                opcode,
                                self,
                            )?;
                            utils::u16_hex_from_str(code_lower_text).ok().map(|code_lower| {
                                (code_upper, code_lower)
                            })
                        } else {
                            Some((code_upper, 0))
                        }
                    }
                };

                parsed.map(|(code_upper, code_lower)| reshift_pair((EncodedFieldMask::code_upper, code_upper.into()), (EncodedFieldMask::code_lower, code_lower.into())))
            }

            Self::core_code_lower => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u16_hex_from_str(text).ok().map(Into::into)
            }

            Self::core_copraw => None /*Self::core_copraw(instr.word() & utils::bitmask(0, 25))*/,

            Self::core_label | Self::core_branch_target_label => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text == "." && next_token == Some(Token::Text("+")) {
                    next_token = Some(Token::Comma);
                    let num_text = request_next_text(
                        token_stream,
                        &mut next_token,
                        opcode,
                        self,
                    )?;

                    let Some(num) = utils::i32_hex_from_str(num_text).ok() else {
                        return Err(EncodingError::UnrecognizedOperand(opcode, num_text, None, self));
                    };

                    let sum = if let Some(tok) = next_token {
                        match bracketed_text_from_token(tok, opcode, self, BracketType::Parenthesis).ok() {
                            None => num,
                            Some(("+", expr)) => {
                                let num2 = if let Some((num2_text, num3_text)) = expr.split_once("<<") {
                                    let num2 = utils::i32_hex_from_str(num2_text.trim()).map_err(|_| EncodingError::UnrecognizedOperand(opcode, num2_text.trim(), None, self))?;
                                    let num3 = utils::u32_hex_from_str(num3_text.trim()).map_err(|_| EncodingError::UnrecognizedOperand(opcode, num3_text.trim(), None, self))?;
                                    num2.wrapping_shl(num3)
                                } else {
                                    utils::i32_hex_from_str(expr.trim()).map_err(|_| EncodingError::UnrecognizedOperand(opcode, expr.trim(), None, self))?
                                };
                                next_token = Some(Token::Comma);
                                num + num2
                            }
                            Some(_) => num,
                        }
                    } else {
                        num
                    };

                    Some(((sum - 4) >> 2) as i16 as u16 as u32)
                } else if text.starts_with("func_") && text.len() >= 5+8 {
                    u32::from_str_radix(&text[5..5+8], 16).ok().map(|x| x >> 2)
                } else {
                    utils::u32_hex_from_str(text).ok().map(|x| (x - 4) >> 2)
                }
            },

            Self::core_imm_i16 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::i16_hex_from_str(text).ok().map(|x| (x as u16).into())
            }
            Self::core_imm_u16 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u16_hex_from_str(text).ok().map(Into::into)
            }

            Self::core_imm_rs => {
                let (imm_text, reg_text) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                utils::i16_hex_from_str(imm_text).ok().and_then(|imm| {
                    regval::<Gpr>(reg_text, abi, allow_dollarless).map(|rs| {
                        reshift_pair((EncodedFieldMask::rs, rs), (EncodedFieldMask::immediate, (imm as u16).into()))
                    })
                })
            },

            Self::core_maybe_rd_rs => {
                let text = operand_text_from_token(token, opcode, self)?;
                match regval::<Gpr>(text, abi, allow_dollarless) {
                    None => None,
                    Some(maybe_rd) => {
                        let (rd, rs) = if next_token != Some(Token::Comma) {
                            (Gpr::ra.as_index() as u32, maybe_rd)
                        } else {
                            let rs = encode_next_reg::<Gpr>(
                                token_stream,
                                &mut next_token,
                                opcode,
                                self,
                                abi,
                                allow_dollarless
                            )?;

                            (maybe_rd, rs)
                        };

                        Some(reshift_pair((EncodedFieldMask::rd, rd), (EncodedFieldMask::rs, rs)))
                    }
                }
            }

            Self::core_maybe_zero_rs => {
                let text = operand_text_from_token(token, opcode, self)?;
                match regval::<Gpr>(text, abi, allow_dollarless) {
                    None => None,
                    Some(maybe_zero) => {
                        Some(if maybe_zero != 0 {
                            maybe_zero
                        } else {
                            encode_next_reg::<Gpr>(
                                token_stream,
                                &mut next_token,
                                opcode,
                                self,
                                abi,
                                allow_dollarless
                            )?
                        })
                    }
                }
            },

            #[cfg(feature = "RSP")]
            Self::rsp_cop0d => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<RspCop0>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "RSP")]
            Self::rsp_cop2cd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<RspCop2Control>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vs | Self::rsp_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<RspVector>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementhigh => {
                // The brackected argument is optional, so we need to check which one we have here.
                let (vt, element) = match bracketed_text_from_token(token, opcode, self, BracketType::Brackets).ok() {
                    Some((vt, element)) => {
                        let element = parse_rsp_element_hq(element, opcode, self)?;

                        (vt, element)
                    }
                    None => {
                        let text = operand_text_from_token(token, opcode, self)?;
                        (text, 0)
                    }
                };

                let vt = regval::<RspVector>(vt, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, vt, Some((vt, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vt, vt), (EncodedFieldMask::rsp_elementhigh, element.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementlow => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Brackets)?;
                let vt = regval::<RspVector>(left, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Brackets)), self,
                ))?;
                let element = utils::u8_hex_from_str(right).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, right, Some((right, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vt, vt), (EncodedFieldMask::rsp_elementlow, element.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vd_de => {
                // The brackected argument is optional, so we need to check which one we have here.
                let (vd, de) = match bracketed_text_from_token(token, opcode, self, BracketType::Brackets).ok() {
                    Some((vd, de)) => {
                        let element = parse_rsp_element_hq(de, opcode, self)?;

                        (vd, element)
                    }
                    None => {
                        let text = operand_text_from_token(token, opcode, self)?;
                        (text, 0)
                    }
                };

                let vd = regval::<RspVector>(vd, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, vd, Some((vd, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vd, vd), (EncodedFieldMask::rsp_de, de.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vs_index => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Brackets)?;
                let vs = regval::<RspVector>(left, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Brackets)), self,
                ))?;
                let index = utils::u8_hex_from_str(right).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, right, Some((right, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vs, vs), (EncodedFieldMask::rsp_index, index.into())))
            }

            #[cfg(feature = "RSP")]
            Self::rsp_offset7_rs => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                let offset = utils::u8_hex_from_str(left).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;
                let rs = regval::<Gpr>(right, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_offset, offset.into()), (EncodedFieldMask::rs, rs)))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset8_rs => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                let offset = utils::u8_hex_from_str(left).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))? >> 1;
                let rs = regval::<Gpr>(right, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_offset, offset.into()), (EncodedFieldMask::rs, rs)))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset9_rs => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                let offset = utils::u16_hex_from_str(left).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))? >> 2;
                let rs = regval::<Gpr>(right, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_offset, offset.into()), (EncodedFieldMask::rs, rs)))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset10_rs => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                let offset = utils::u16_hex_from_str(left).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))? >> 3;
                let rs = regval::<Gpr>(right, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_offset, offset.into()), (EncodedFieldMask::rs, rs)))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset11_rs => {
                let (left, right) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                let offset = utils::u16_hex_from_str(left).map_err(|_| EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))? >> 4;
                let rs = regval::<Gpr>(right, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
                    opcode, left, Some((right, BracketType::Parenthesis)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_offset, offset.into()), (EncodedFieldMask::rs, rs)))
            }

            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_sf | Self::r3000gte_mx | Self::r3000gte_v | Self::r3000gte_cv | Self::r3000gte_lm => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }

            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vs | Self::r4000allegrex_s_vt | Self::r4000allegrex_s_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexS>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vt_imm => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexS>(text, abi, allow_dollarless).map(|x| {
                    let upper = x >> 5;
                    let lower = x & utils::bitmask(0, 5);

                    reshift_pair((EncodedFieldMask::r4000allegrex_vt_6_imm_upper, upper), (EncodedFieldMask::r4000allegrex_vt_imm_lower, lower))
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vd_imm => None /*{
                Self::r4000allegrex_s_vd_imm(field.r4000allegrex_s_vd_imm_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vs | Self::r4000allegrex_p_vt | Self::r4000allegrex_p_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexV2D>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vs | Self::r4000allegrex_t_vt | Self::r4000allegrex_t_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexV3D>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vs | Self::r4000allegrex_q_vt | Self::r4000allegrex_q_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexV4D>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vt_imm => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexV4D>(text, abi, allow_dollarless).map(|x| {
                    let upper = x >> 5;
                    let lower = x & utils::bitmask(0, 5);

                    reshift_pair((EncodedFieldMask::r4000allegrex_vt_6_imm_upper, upper), (EncodedFieldMask::r4000allegrex_vt_imm_lower, lower))
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs | Self::r4000allegrex_mp_vt | Self::r4000allegrex_mp_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM2x2>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs_transpose => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM2x2>(text, abi, allow_dollarless).map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs | Self::r4000allegrex_mt_vt | Self::r4000allegrex_mt_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM3x3>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs_transpose => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM3x3>(text, abi, allow_dollarless).map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs | Self::r4000allegrex_mq_vt | Self::r4000allegrex_mq_vd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM4x4>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs_transpose => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexM4x4>(text, abi, allow_dollarless).map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cs => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexVfpuControl>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexVfpuControl>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_pos => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(|x| (x - 1).into())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size_plus_pos => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_imm3 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_rs => {
                let (offset14, reg_text) = bracketed_text_from_token(token, opcode, self, BracketType::Parenthesis)?;
                utils::u16_hex_from_str(offset14).ok().and_then(|imm| {
                    regval::<Gpr>(reg_text, abi, allow_dollarless).map(|rs| {
                        reshift_pair((EncodedFieldMask::rs, rs), (EncodedFieldMask::immediate, (imm >> 2).into()))
                    })
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_rs_maybe_wb => None /*{
                Self::r4000allegrex_offset14_rs_maybe_wb(
                    field.r4000allegrex_offset14_impl(),
                    field.rs_impl(),
                    field.r4000allegrex_wb_impl(),
                )
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                let text = operand_text_from_token(token, opcode, self)?;
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexS>(
                            token_stream,
                            &mut next_token,
                            opcode,
                            self,
                            abi,
                            allow_dollarless,
                            cond,
                        )?)
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                let text = operand_text_from_token(token, opcode, self)?;
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV2D>(
                            token_stream,
                            &mut next_token,
                            opcode,
                            self,
                            abi,
                            allow_dollarless,
                            cond,
                        )?)
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                let text = operand_text_from_token(token, opcode, self)?;
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV3D>(
                            token_stream,
                            &mut next_token,
                            opcode,
                            self,
                            abi,
                            allow_dollarless,
                            cond,
                        )?)
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                let text = operand_text_from_token(token, opcode, self)?;
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV4D>(
                            token_stream,
                            &mut next_token,
                            opcode,
                            self,
                            abi,
                            allow_dollarless,
                            cond,
                        )?)
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vconstant => None /*{
                Self::r4000allegrex_vconstant(field.r4000allegrex_vconstant_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_power_of_two => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vfpu_cc_bit => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_bn => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u8_hex_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_int16 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::i16_hex_from_str(text).ok().map(|x| (x as u16).into())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_float16 => {
                let text = operand_text_from_token(token, opcode, self)?;
                let float = text.parse::<f32>();
                float.ok().filter(|x| (-65504.0..=65504.0).contains(x) || x.is_nan() || x.is_infinite()).map(|x| {
                    utils::floatrepr_16_from_32(x.to_bits()).into()
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vrot_code => {
                let text = bracket_solo_from_token(token, opcode, self, BracketType::Brackets)?;
                // TODO: what to do with the duplicated entries?
                match text {
                    "C,S"  => Some(0),
                    "S,C"  => Some(1),
                    "S,0"  => Some(2),
                    //"S,0"  => Some(3),
                    //"C,S"  => Some(4),
                    //"S,C"  => Some(5),
                    "0,S"  => Some(6),
                    //"0,S"  => Some(7),
                    "C,0"  => Some(8),
                    "0,C"  => Some(9),
                    "S,S"  => Some(10),
                    "0,0"  => Some(11),
                    //"C,0"  => Some(12),
                    //"0,C"  => Some(13),
                    //"0,0"  => Some(14),
                    //"S,S"  => Some(15),
                    "C,-S" => Some(16),
                    "-S,C" => Some(17),
                    "-S,0" => Some(18),
                    //"-S,0" => Some(19),
                    //"C,-S" => Some(20),
                    //"-S,C" => Some(21),
                    "0,-S" => Some(22),
                    //"0,-S" => Some(23),
                    //"C,0"  => Some(24),
                    //"0,C"  => Some(25),
                    "-S,-S"=> Some(26),
                    //"0,0"  => Some(27),
                    //"C,0"  => Some(28),
                    //"0,C"  => Some(29),
                    //"0,0"  => Some(30),
                    //"-S,-S"=> Some(31),
                    _ => None,
                }
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vrot_code => {
                let text = bracket_solo_from_token(token, opcode, self, BracketType::Brackets)?;
                // TODO: what to do with the duplicated entries?
                match text {
                    "C,S,S" => Some(0),
                    "S,C,0" => Some(1),
                    "S,0,C" => Some(2),
                    "S,0,0" => Some(3),
                    "C,S,0" => Some(4),
                    "S,C,S" => Some(5),
                    "0,S,C" => Some(6),
                    "0,S,0" => Some(7),
                    "C,0,S" => Some(8),
                    "0,C,S" => Some(9),
                    "S,S,C" => Some(10),
                    "0,0,S" => Some(11),
                    "C,0,0" => Some(12),
                    "0,C,0" => Some(13),
                    "0,0,C" => Some(14),
                    "S,S,S" => Some(15),
                    "C,-S,-S" => Some(16),
                    "-S,C,0" => Some(17),
                    "-S,0,C" => Some(18),
                    "-S,0,0" => Some(19),
                    "C,-S,0" => Some(20),
                    "-S,C,-S" => Some(21),
                    "0,-S,C" => Some(22),
                    "0,-S,0" => Some(23),
                    "C,0,-S" => Some(24),
                    "0,C,-S" => Some(25),
                    "-S,-S,C" => Some(26),
                    "0,0,-S" => Some(27),
                    //"C,0,0" => Some(28),
                    //"0,C,0" => Some(29),
                    //"0,0,C" => Some(30),
                    "-S,-S,-S" => Some(31),
                    _ => None,
                }
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vrot_code => {
                let text = bracket_solo_from_token(token, opcode, self, BracketType::Brackets)?;
                // TODO: what to do with the duplicated entries?
                match text {
                    "C,S,S,S" => Some(0),
                    "S,C,0,0" => Some(1),
                    "S,0,C,0" => Some(2),
                    "S,0,0,C" => Some(3),
                    "C,S,0,0" => Some(4),
                    "S,C,S,S" => Some(5),
                    "0,S,C,0" => Some(6),
                    "0,S,0,C" => Some(7),
                    "C,0,S,0" => Some(8),
                    "0,C,S,0" => Some(9),
                    "S,S,C,S" => Some(10),
                    "0,0,S,C" => Some(11),
                    "C,0,0,S" => Some(12),
                    "0,C,0,S" => Some(13),
                    "0,0,C,S" => Some(14),
                    "S,S,S,C" => Some(15),
                    "C,-S,-S,-S" => Some(16),
                    "-S,C,0,0" => Some(17),
                    "-S,0,C,0" => Some(18),
                    "-S,0,0,C" => Some(19),
                    "C,-S,0,0" => Some(20),
                    "-S,C,-S,-S" => Some(21),
                    "0,-S,C,0" => Some(22),
                    "0,-S,0,C" => Some(23),
                    "C,0,-S,0" => Some(24),
                    "0,C,-S,0" => Some(25),
                    "-S,-S,C,-S" => Some(26),
                    "0,0,-S,C" => Some(27),
                    "C,0,0,-S" => Some(28),
                    "0,C,0,-S" => Some(29),
                    "0,0,C,-S" => Some(30),
                    "-S,-S,-S,C" => Some(31),
                    _ => None,
                }
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpx => None /*Self::r4000allegrex_wpx(field.r4000allegrex_wpx_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpy => None /*Self::r4000allegrex_wpy(field.r4000allegrex_wpy_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpz => None /*Self::r4000allegrex_wpz(field.r4000allegrex_wpz_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpw => None /*Self::r4000allegrex_wpw(field.r4000allegrex_wpw_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpx => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexPrefixSrc>(text, abi, allow_dollarless).map(|x| {
                    let a = (x & utils::bitmask(4, 1)) << 16;
                    let b = (x & utils::bitmask(3, 1)) << 12;
                    let c = (x & utils::bitmask(2, 1)) << 8;
                    let d = x & utils::bitmask(0, 2);

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpy => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexPrefixSrc>(text, abi, allow_dollarless).map(|x| {
                    let a = (x & utils::bitmask(4, 1)) << 17;
                    let b = (x & utils::bitmask(3, 1)) << 13;
                    let c = (x & utils::bitmask(2, 1)) << 9;
                    let d = (x & utils::bitmask(0, 2)) << 2;

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpz => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexPrefixSrc>(text, abi, allow_dollarless).map(|x| {
                    let a = (x & utils::bitmask(4, 1)) << 18;
                    let b = (x & utils::bitmask(3, 1)) << 14;
                    let c = (x & utils::bitmask(2, 1)) << 10;
                    let d = (x & utils::bitmask(0, 2)) << 4;

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpw => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R4000AllegrexPrefixSrc>(text, abi, allow_dollarless).map(|x| {
                    let a = (x & utils::bitmask(4, 1)) << 19;
                    let b = (x & utils::bitmask(3, 1)) << 15;
                    let c = (x & utils::bitmask(2, 1)) << 11;
                    let d = (x & utils::bitmask(0, 2)) << 6;

                    a | b | c | d
                })
            }

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_I => {
                let text = operand_text_from_token(token, opcode, self)?;
                matches!(text, "I" | "$I").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_Q => {
                let text = operand_text_from_token(token, opcode, self)?;
                matches!(text, "Q" | "$Q").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_R => {
                let text = operand_text_from_token(token, opcode, self)?;
                matches!(text, "R" | "$R").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACC => None /*Self::r5900ee_ACC()*/,

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm5 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::i8_hex_from_str(text).ok().map(|x| (x as u8).into())
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm15 => {
                let text = operand_text_from_token(token, opcode, self)?;
                utils::u32_hex_from_str(text).ok().map(|x| x >> 3)
            }

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfs | Self::r5900ee_vft | Self::r5900ee_vfd => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R5900EEVF>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis | Self::r5900ee_vit | Self::r5900ee_vid => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R5900EEVI>(text, abi, allow_dollarless)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACCxyzw => {
                let text = operand_text_from_token(token, opcode, self)?;
                matches!(text, "ACC" | "$ACC").then_some(0)
                /*
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
                */
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsxyzw | Self::r5900ee_vftxyzw | Self::r5900ee_vfdxyzw => {
                let text = operand_text_from_token(token, opcode, self)?;
                regval::<R5900EEVF>(text, abi, allow_dollarless)
                /*
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
                */
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftn => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let n = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], abi, allow_dollarless).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vft, reg), (EncodedFieldMask::r5900ee_n, n))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsl => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let l = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], abi, allow_dollarless).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vfs, reg), (EncodedFieldMask::r5900ee_l, l))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftm => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let m = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], abi, allow_dollarless).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vft, reg), (EncodedFieldMask::r5900ee_m, m))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_predecr | Self::r5900ee_vit_predecr => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.starts_with("(--") && text.ends_with(')') {
                    regval::<R5900EEVI>(&text[3..text.len()-1], abi, allow_dollarless)
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_postincr | Self::r5900ee_vit_postincr => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.starts_with('(') && text.ends_with("++)") {
                    regval::<R5900EEVI>(&text[1..text.len()-3], abi, allow_dollarless)
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_parenthesis => {
                let text = operand_text_from_token(token, opcode, self)?;
                if text.starts_with('(') && text.ends_with(')') {
                    regval::<R5900EEVI>(&text[1..text.len()-1], abi, allow_dollarless)
                } else {
                    None
                }
            }
        };

        let encoded = if let Some(val) = val {
            self.mask().unshift(val)
        } else {
            let err = match token {
                Token::End => EncodingError::EndTokenInsteadOfOperand(opcode, self),
                Token::Comma => EncodingError::CommaInsteadOfOperand(opcode, self),
                Token::Text(text) => EncodingError::UnrecognizedOperand(opcode, text, None, self),
                Token::Bracketed(left, right, bracket_type) => EncodingError::UnrecognizedOperand(
                    opcode,
                    left,
                    Some((right, bracket_type)),
                    self,
                ),
                Token::BracketSolo(text, bracket_type) => {
                    EncodingError::UnrecognizedOperand(opcode, "", Some((text, bracket_type)), self)
                }
            };

            return Err(err);
        };

        EncodedOperandBits::new(encoded, next_token, opcode, self)
    }
}

fn regval<R>(name: &str, abi: Abi, allow_dollarless: bool) -> Option<u32>
where
    R: Register,
{
    R::from_name(name, abi, allow_dollarless).map(|x| x.as_index() as u32)
}

const fn operand_text_from_token<'s>(
    token: Token<'s>,
    opcode: Opcode,
    operand: Operand,
) -> Result<&'s str, EncodingError<'s>> {
    match token {
        Token::Text(text) => Ok(text),

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(opcode, operand)),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(opcode, operand)),
        Token::Bracketed(left, right, bracket_type) => {
            Err(EncodingError::BracketedInsteadOfSingleOperand(
                opcode,
                operand,
                left,
                right,
                bracket_type,
            ))
        }
        Token::BracketSolo(text, bracket_type) => Err(
            EncodingError::BracketSoloInsteadOfSingleOperand(opcode, operand, text, bracket_type),
        ),
    }
}

fn bracketed_text_from_token<'s>(
    token: Token<'s>,
    opcode: Opcode,
    operand: Operand,
    required_bracket_type: BracketType,
) -> Result<(&'s str, &'s str), EncodingError<'s>> {
    match token {
        Token::Bracketed(left, right, bracket_type) => {
            if bracket_type == required_bracket_type {
                Ok((left, right))
            } else {
                Err(EncodingError::WrongBracketedOperand(
                    opcode,
                    operand,
                    left,
                    right,
                    bracket_type,
                    required_bracket_type,
                ))
            }
        }

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(opcode, operand)),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(opcode, operand)),
        Token::Text(text) => Err(EncodingError::TextInsteadOfBracketedOperand(
            opcode,
            operand,
            text,
            required_bracket_type,
        )),
        Token::BracketSolo(text, bracket_type) => {
            Err(EncodingError::BracketSoloInsteadOfBracketedOperand(
                opcode,
                operand,
                text,
                bracket_type,
                required_bracket_type,
            ))
        }
    }
}

fn bracket_solo_from_token<'s>(
    token: Token<'s>,
    opcode: Opcode,
    operand: Operand,
    required_bracket_type: BracketType,
) -> Result<&'s str, EncodingError<'s>> {
    match token {
        Token::BracketSolo(text, bracket_type) => {
            if bracket_type == required_bracket_type {
                Ok(text)
            } else {
                Err(EncodingError::WrongBracketedOperand(
                    opcode,
                    operand,
                    "",
                    text,
                    bracket_type,
                    required_bracket_type,
                ))
            }
        }

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(opcode, operand)),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(opcode, operand)),
        Token::Text(text) => Err(EncodingError::TextInsteadOfBracketedOperand(
            opcode,
            operand,
            text,
            required_bracket_type,
        )),
        Token::Bracketed(left, right, bracket_type) => {
            Err(EncodingError::BracketedInsteadOfBracketSoloOperand(
                opcode,
                operand,
                left,
                right,
                bracket_type,
                required_bracket_type,
            ))
        }
    }
}

fn request_next_text<'s>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<Token<'s>>,
    opcode: Opcode,
    operand: Operand,
) -> Result<&'s str, EncodingError<'s>> {
    if next_token != &Some(Token::Comma) {
        return Err(EncodingError::MissingCommaInComposedOperand(
            opcode, operand,
        ));
    }

    let Some((token_aux, next_token_aux)) = token_stream.next() else {
        return Err(EncodingError::RanOutOfTokens(opcode, operand));
    };
    *next_token = next_token_aux;

    operand_text_from_token(token_aux, opcode, operand)
}

fn request_next_reg<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<Token<'s>>,
    opcode: Opcode,
    operand: Operand,
    abi: Abi,
    allow_dollarless: bool,
) -> Result<R, EncodingError<'s>>
where
    R: Register,
{
    let text = request_next_text(token_stream, next_token, opcode, operand)?;

    R::from_name(text, abi, allow_dollarless).ok_or(EncodingError::UnrecognizedOperand(
        opcode, text, None, operand,
    ))
}

fn encode_next_reg<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<Token<'s>>,
    opcode: Opcode,
    operand: Operand,
    abi: Abi,
    allow_dollarless: bool,
) -> Result<u32, EncodingError<'s>>
where
    R: Register,
{
    request_next_reg(
        token_stream,
        next_token,
        opcode,
        operand,
        abi,
        allow_dollarless,
    )
    .map(|reg: R| reg.as_index() as u32)
}

#[cfg(feature = "RSP")]
fn parse_rsp_element_hq<'s>(
    text: &'s str,
    opcode: Opcode,
    operand: Operand,
) -> Result<u8, EncodingError<'s>> {
    let w = text.ends_with('w');
    let h = text.ends_with('h');
    let q = text.ends_with('q');

    let (element, suffix_less) = if w || h || q {
        (&text[..text.len() - 1], false)
    } else {
        (text, true)
    };

    utils::u8_hex_from_str(element)
        .map_err(|_| EncodingError::UnrecognizedOperand(opcode, text, None, operand))
        .map(|x| {
            if w || suffix_less {
                x | 8
            } else if h {
                x | 4
            } else if q {
                x | 2
            } else {
                x
            }
        })
}

#[cfg(feature = "R4000ALLEGREX")]
fn identify_r4000allegrex_vcmp_registers<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<Token<'s>>,
    opcode: Opcode,
    operand: Operand,
    abi: Abi,
    allow_dollarless: bool,
    cond: R4000AllegrexVCond,
) -> Result<(R, R), EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let (vs, vt) = match cond {
        R4000AllegrexVCond::fl | R4000AllegrexVCond::tr => {
            // The two arguments may be omitted if they are zero.
            if next_token != &Some(Token::Comma) {
                (R::default(), R::default())
            } else {
                let vs = request_next_reg(
                    token_stream,
                    next_token,
                    opcode,
                    operand,
                    abi,
                    allow_dollarless,
                )?;
                let vt = request_next_reg(
                    token_stream,
                    next_token,
                    opcode,
                    operand,
                    abi,
                    allow_dollarless,
                )?;

                (vs, vt)
            }
        }
        R4000AllegrexVCond::eq
        | R4000AllegrexVCond::lt
        | R4000AllegrexVCond::le
        | R4000AllegrexVCond::ne
        | R4000AllegrexVCond::ge
        | R4000AllegrexVCond::gt => {
            let vs = request_next_reg(
                token_stream,
                next_token,
                opcode,
                operand,
                abi,
                allow_dollarless,
            )?;
            let vt = request_next_reg(
                token_stream,
                next_token,
                opcode,
                operand,
                abi,
                allow_dollarless,
            )?;

            (vs, vt)
        }
        R4000AllegrexVCond::ez
        | R4000AllegrexVCond::en
        | R4000AllegrexVCond::ei
        | R4000AllegrexVCond::es
        | R4000AllegrexVCond::nz
        | R4000AllegrexVCond::nn
        | R4000AllegrexVCond::ni
        | R4000AllegrexVCond::ns => {
            let vs = request_next_reg(
                token_stream,
                next_token,
                opcode,
                operand,
                abi,
                allow_dollarless,
            )?;

            /* The last register may be omitted if it is zero */
            let vt = if next_token != &Some(Token::Comma) {
                R::default()
            } else {
                request_next_reg(
                    token_stream,
                    next_token,
                    opcode,
                    operand,
                    abi,
                    allow_dollarless,
                )?
            };

            (vs, vt)
        }
    };

    Ok((vs, vt))
}

#[cfg(feature = "R4000ALLEGREX")]
fn encode_r4000allegrex_vcmp_registers<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<Token<'s>>,
    opcode: Opcode,
    operand: Operand,
    abi: Abi,
    allow_dollarless: bool,
    cond: R4000AllegrexVCond,
) -> Result<u32, EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let (vs, vt) = identify_r4000allegrex_vcmp_registers::<R>(
        token_stream,
        next_token,
        opcode,
        operand,
        abi,
        allow_dollarless,
        cond,
    )?;

    let cond_bits = EncodedFieldMask::r4000allegrex_vcmp_cond.unshift(cond.as_index() as u32);
    let vs_bits = EncodedFieldMask::r4000allegrex_vs.unshift(vs.as_index() as u32);
    let vt_bits = EncodedFieldMask::r4000allegrex_vt.unshift(vt.as_index() as u32);
    Ok(cond_bits | vs_bits | vt_bits)
}

const fn reshift_pair(
    (mask_a, mut value_a): (EncodedFieldMask, u32),
    (mask_b, mut value_b): (EncodedFieldMask, u32),
) -> u32 {
    value_a = mask_a.unshift(value_a);
    value_b = mask_b.unshift(value_b);

    mask_a.union(mask_b).get_shifted(value_a | value_b)
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[must_use]
pub(crate) enum EncodedOperandBits {
    EndBits(u32),
    ContinueBits(u32),
}

impl EncodedOperandBits {
    const fn new<'s>(
        bits: u32,
        next_token: Option<Token<'s>>,
        opcode: Opcode,
        operand: Operand,
    ) -> Result<Self, EncodingError<'s>> {
        match next_token {
            None | Some(Token::End) => Ok(Self::EndBits(bits)),
            Some(Token::Comma) => Ok(Self::ContinueBits(bits)),

            Some(Token::Text(t)) => Err(EncodingError::TokenInsteadOfCommaEnd(opcode, operand, t)),
            Some(Token::Bracketed(left, right, bracket_type)) => {
                Err(EncodingError::BracketedInsteadOfCommaEnd(
                    opcode,
                    operand,
                    left,
                    right,
                    bracket_type,
                ))
            }
            Some(Token::BracketSolo(text, bracket_type)) => Err(
                EncodingError::BracketSoloInsteadOfCommaEnd(opcode, operand, text, bracket_type),
            ),
        }
    }
}
