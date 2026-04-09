/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::encoded_field_mask::EncodedFieldMask;
use crate::opcodes::Opcode;
use crate::operands::Operand;
use crate::registers::*;
#[cfg(feature = "R4000ALLEGREX")]
use crate::registers_meta::R4000AllegrexVectorRegister;
use crate::registers_meta::Register;
use crate::utils::{self, iter::DoubleOptIterator};

use super::token::{BracketType, Token, TokenDottedText, Tokenize};
use super::EncoderFlags;
use super::EncodingError;

pub(crate) struct OperandEncoderFlags<'flgs> {
    encoder_flags: &'flgs EncoderFlags,
    opcode: Opcode,
}

impl<'flgs> OperandEncoderFlags<'flgs> {
    #[must_use]
    pub const fn new(encoder_flags: &'flgs EncoderFlags, opcode: Opcode) -> Self {
        Self {
            encoder_flags,
            opcode,
        }
    }

    #[must_use]
    pub const fn abi(&self) -> Abi {
        self.encoder_flags.instruction_flags().abi()
    }

    #[must_use]
    pub const fn allow_dollarless(&self) -> bool {
        self.encoder_flags.allow_dollarless()
    }

    #[must_use]
    #[cfg(feature = "R5900EE")]
    pub const fn r5900ee_prodg_sn_as_inverted_regs(&self) -> bool {
        self.encoder_flags.r5900ee_prodg_sn_as_inverted_regs()
    }
}

impl Operand {
    #[expect(clippy::cognitive_complexity)]
    pub(crate) fn encode_to_bits<'s>(
        self,
        token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
        flags: &OperandEncoderFlags,
    ) -> Result<EncodedOperandBits, EncodingError<'s>> {
        let Some(((mut token, starting_index, mut ending_index), mut next_token)) =
            token_stream.next()
        else {
            return Err(EncodingError::RanOutOfTokens(flags.opcode, self));
        };
        let mut mask = self.mask();

        // Hacky way to workaround unused_mut warning that gets triggered under
        // some feature flags combinations.
        #[allow(dead_code)]
        #[expect(clippy::self_assignment)]
        {
            token = token;
            mask = mask;
        }

        let val = match self {
            Self::ALL_EMPTY => None,
            Self::core_rs | Self::core_rt | Self::core_rd => {
                regval_from_text_token::<Gpr>(token, flags, self)?
            }
            Self::core_sa => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            Self::core_zero => None /*Self::core_zero()*/,
            Self::core_cop0d => {
                regval_from_text_token::<Cop0>(token, flags, self)?
            }
            Self::core_cop0cd => {
                regval_from_text_token::<Cop0Control>(token, flags, self)?
            }
            Self::core_fs | Self::core_ft | Self::core_fd => {
                regval_from_text_token::<Cop1>(token, flags, self)?
            }
            Self::core_cop1cs => {
                regval_from_text_token::<Cop1Control>(token, flags, self)?
            }
            Self::core_cop2t | Self::core_cop2d => {
                regval_from_text_token::<Cop2>(token, flags, self)?
            }
            Self::core_cop2cd => {
                regval_from_text_token::<Cop2Control>(token, flags, self)?
            }

            Self::core_op => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            Self::core_hint => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }

            Self::core_code => {
                let text = operand_text_from_token(token, flags, self)?;
                let parsed = match utils::hex_num::u16_from_str(text).ok() {
                    None => None,
                    Some(code_upper) => {
                        if matches!(next_token, Some((Token::Comma, _, _))) {
                            let code_lower_text = request_next_text(
                                token_stream,
                                &mut next_token,
                                flags,
                                self,
                                &mut ending_index,
                            )?;
                            utils::hex_num::u16_from_str(code_lower_text).ok().map(|code_lower| {
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
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u16_from_str(text).ok().map(Into::into)
            }

            Self::core_copraw => None /*Self::core_copraw(instr.word() & utils::bitmask(0, 25))*/,

            Self::core_label | Self::core_branch_target_label => {
                let text = operand_text_from_token(token, flags, self)?;
                if text == "." && matches!(next_token, Some((Token::Text("+"), _, _))) {
                    next_token = Some((Token::Comma, ending_index+1, ending_index+2));
                    let num_text = request_next_text(
                        token_stream,
                        &mut next_token,
                        flags,
                        self,
                        &mut ending_index,
                    )?;

                    let Some(num) = utils::hex_num::i32_from_str(num_text).ok() else {
                        return Err(EncodingError::UnrecognizedOperand(flags.opcode, num_text, None, self));
                    };

                    let sum = if let Some((tok, _, new_end)) = next_token {
                        ending_index = new_end;

                        match bracketed_text_from_token(tok, flags, self, BracketType::Parenthesis).ok() {
                            None => num,
                            Some(("+", expr)) => {
                                let num2 = if let Some((num2_text, num3_text)) = expr.split_once("<<") {
                                    let num2 = utils::hex_num::i32_from_str(num2_text.trim()).map_err(|_| EncodingError::UnrecognizedOperand(flags.opcode, num2_text.trim(), None, self))?;
                                    let num3 = utils::hex_num::u32_from_str(num3_text.trim()).map_err(|_| EncodingError::UnrecognizedOperand(flags.opcode, num3_text.trim(), None, self))?;
                                    num2.wrapping_shl(num3)
                                } else {
                                    utils::hex_num::i32_from_str(expr.trim()).map_err(|_| EncodingError::UnrecognizedOperand(flags.opcode, expr.trim(), None, self))?
                                };
                                next_token = Some((Token::Comma, new_end, new_end+1));
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
                    // If the text can't be interpreted as a number then do zero instead of erroring.
                    Some(utils::hex_num::u32_from_str(text).map_or(0, |x| x.saturating_sub(4) >> 2))
                }
            },

            Self::core_imm_i16 => {
                if let Ok((reloc_operator, _sym)) = bracketed_text_from_token(token, flags, self, BracketType::Parenthesis) {
                    // If there's a reloc operator then use zero as the imm value instead of rejecting the whole instruction.
                    reloc_operator.starts_with('%').then_some(0)
                } else {
                    let text = operand_text_from_token(token, flags, self)?;
                    utils::hex_num::i16_from_str(text).ok().map(|x| (x as u16).into())
                }
            }
            Self::core_imm_u16 => {
                if let Ok((reloc_operator, _sym)) = bracketed_text_from_token(token, flags, self, BracketType::Parenthesis) {
                    // If there's a reloc operator then use zero as the imm value instead of rejecting the whole instruction.
                    reloc_operator.starts_with('%').then_some(0)
                } else {
                    let text = operand_text_from_token(token, flags, self)?;
                    utils::hex_num::u16_from_str(text).ok().map(Into::into)
                }
            }

            Self::core_imm_rs => {
                let (imm_text, reg_text) = bracketed_text_from_token(token, flags, self, BracketType::Parenthesis)?;
                utils::hex_num::i16_from_str(imm_text).ok().and_then(|imm| {
                    regval::<Gpr>(reg_text, flags).map(|rs| {
                        reshift_pair((EncodedFieldMask::rs, rs), (EncodedFieldMask::immediate, (imm as u16).into()))
                    })
                })
            },

            Self::core_maybe_rd_rs => {
                let text = operand_text_from_token(token, flags, self)?;
                match regval::<Gpr>(text, flags) {
                    None => None,
                    Some(maybe_rd) => {
                        let (rd, rs) = if !matches!(next_token, Some((Token::Comma, _, _))) {
                            (Gpr::ra.as_index() as u32, maybe_rd)
                        } else {
                            let rs = encode_next_reg::<Gpr>(
                                token_stream,
                                &mut next_token,
                                flags,
                                self,
                                &mut ending_index,
                            )?;
                            (maybe_rd, rs)
                        };

                        Some(reshift_pair((EncodedFieldMask::rd, rd), (EncodedFieldMask::rs, rs)))
                    }
                }
            }

            Self::core_maybe_zero_rs => {
                let text = operand_text_from_token(token, flags, self)?;
                match regval::<Gpr>(text, flags) {
                    None => None,
                    Some(maybe_zero) => {
                        Some(if maybe_zero != 0 {
                            maybe_zero
                        } else {
                            encode_next_reg::<Gpr>(
                                token_stream,
                                &mut next_token,
                                flags,
                                self,
                                &mut ending_index,
                            )?
                        })
                    }
                }
            },

            #[cfg(feature = "RSP")]
            Self::rsp_cop0d => {
                regval_from_text_token::<RspCop0>(token, flags, self)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_cop2cd => {
                regval_from_text_token::<RspCop2Control>(token, flags, self)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vs | Self::rsp_vd => {
                regval_from_text_token::<RspVector>(token, flags, self)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementhigh => {
                // The brackected argument is optional, so we need to check which one we have here.
                let (vt, element) = match bracketed_text_from_token(token, flags, self, BracketType::Brackets).ok() {
                    Some((vt, element)) => {
                        let element = parse_rsp_element_hq(element, flags, self)?;

                        (vt, element)
                    }
                    None => {
                        let text = operand_text_from_token(token, flags, self)?;
                        (text, 0)
                    }
                };

                let vt = regval::<RspVector>(vt, flags).ok_or(EncodingError::UnrecognizedOperand(
                    flags.opcode, vt, Some((vt, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vt, vt), (EncodedFieldMask::rsp_elementhigh, element.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementlow => {
                let (left, right) = bracketed_text_from_token(token, flags, self, BracketType::Brackets)?;
                let vt = regval::<RspVector>(left, flags).ok_or(EncodingError::UnrecognizedOperand(
                    flags.opcode, left, Some((right, BracketType::Brackets)), self,
                ))?;
                let element = utils::hex_num::u8_from_str(right).map_err(|_| EncodingError::UnrecognizedOperand(
                    flags.opcode, right, Some((right, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vt, vt), (EncodedFieldMask::rsp_elementlow, element.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vd_de => {
                // The brackected argument is optional, so we need to check which one we have here.
                let (vd, de) = match bracketed_text_from_token(token, flags, self, BracketType::Brackets).ok() {
                    Some((vd, de)) => {
                        let element = parse_rsp_element_hq(de, flags, self)?;
                        (vd, element)
                    }
                    None => {
                        let text = operand_text_from_token(token, flags, self)?;
                        (text, 0)
                    }
                };

                let vd = regval::<RspVector>(vd, flags).ok_or(EncodingError::UnrecognizedOperand(
                    flags.opcode, vd, Some((vd, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vd, vd), (EncodedFieldMask::rsp_de, de.into())))
            }
            #[cfg(feature = "RSP")]
            Self::rsp_vs_index => {
                let (left, right) = bracketed_text_from_token(token, flags, self, BracketType::Brackets)?;
                let vs = regval::<RspVector>(left, flags).ok_or(EncodingError::UnrecognizedOperand(
                    flags.opcode, left, Some((right, BracketType::Brackets)), self,
                ))?;
                let index = utils::hex_num::u8_from_str(right).map_err(|_| EncodingError::UnrecognizedOperand(
                    flags.opcode, right, Some((right, BracketType::Brackets)), self,
                ))?;

                Some(reshift_pair((EncodedFieldMask::rsp_vs, vs), (EncodedFieldMask::rsp_index, index.into())))
            }

            #[cfg(feature = "RSP")]
            Self::rsp_offset7_rs => {
                encode_rsp_offset_rs(token, flags, self, utils::hex_num::u8_from_str, 0)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset8_rs => {
                encode_rsp_offset_rs(token, flags, self, utils::hex_num::u8_from_str, 1)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset9_rs => {
                encode_rsp_offset_rs(token, flags, self, utils::hex_num::u16_from_str, 2)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset10_rs => {
                encode_rsp_offset_rs(token, flags, self, utils::hex_num::u16_from_str, 3)?
            }
            #[cfg(feature = "RSP")]
            Self::rsp_offset11_rs => {
                encode_rsp_offset_rs(token, flags, self, utils::hex_num::u16_from_str, 4)?
            }

            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_gbg | Self::r3000gte_sf | Self::r3000gte_mx | Self::r3000gte_v | Self::r3000gte_cv | Self::r3000gte_lm => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }

            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vs | Self::r4000allegrex_s_vt | Self::r4000allegrex_s_vd => {
                regval_from_text_token::<R4000AllegrexS>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vt_imm => {
                regval_from_text_token::<R4000AllegrexS>(token, flags, self)?.map(|x| {
                    let upper = x >> 5;
                    let lower = x & utils::bitmask(0, 5);

                    reshift_pair((EncodedFieldMask::r4000allegrex_vt_imm_upper, upper), (EncodedFieldMask::r4000allegrex_vt_imm_lower, lower))
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vd_imm => None /*{
                Self::r4000allegrex_s_vd_imm(field.r4000allegrex_s_vd_imm_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vs | Self::r4000allegrex_p_vt | Self::r4000allegrex_p_vd => {
                regval_from_text_token::<R4000AllegrexV2D>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vs | Self::r4000allegrex_t_vt | Self::r4000allegrex_t_vd => {
                regval_from_text_token::<R4000AllegrexV3D>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vs | Self::r4000allegrex_q_vt | Self::r4000allegrex_q_vd => {
                regval_from_text_token::<R4000AllegrexV4D>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vt_imm => {
                regval_from_text_token::<R4000AllegrexV4D>(token, flags, self)?.map(|x| {
                    let upper = x >> 5;
                    let lower = x & utils::bitmask(0, 5);

                    reshift_pair((EncodedFieldMask::r4000allegrex_vt_6_imm_upper, upper), (EncodedFieldMask::r4000allegrex_vt_imm_lower, lower))
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs | Self::r4000allegrex_mp_vt | Self::r4000allegrex_mp_vd => {
                regval_from_text_token::<R4000AllegrexM2x2>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs_transpose => {
                regval_from_text_token::<R4000AllegrexM2x2>(token, flags, self)?.map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs | Self::r4000allegrex_mt_vt | Self::r4000allegrex_mt_vd => {
                regval_from_text_token::<R4000AllegrexM3x3>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs_transpose => {
                regval_from_text_token::<R4000AllegrexM3x3>(token, flags, self)?.map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs | Self::r4000allegrex_mq_vt | Self::r4000allegrex_mq_vd => {
                regval_from_text_token::<R4000AllegrexM4x4>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs_transpose => {
                regval_from_text_token::<R4000AllegrexM4x4>(token, flags, self)?.map(|x| x ^ 0x20)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cs => {
                regval_from_text_token::<R4000AllegrexVfpuControl>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cd => {
                regval_from_text_token::<R4000AllegrexVfpuControl>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_pos => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(|x| (x - 1).into())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size_plus_pos => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::i8_from_str(text).ok().map(|x| (x as u8).into())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_imm3 => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_rs => {
                let (offset14, reg_text) = bracketed_text_from_token(token, flags, self, BracketType::Parenthesis)?;
                utils::hex_num::u16_from_str(offset14).ok().and_then(|imm| {
                    regval::<Gpr>(reg_text, flags).map(|rs| {
                        reshift_pair((EncodedFieldMask::r4000allegrex_offset14, (imm >> 2).into()), (EncodedFieldMask::rs, rs))
                    })
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_rs_maybe_wb => {
                let (offset14, reg_text) = bracketed_text_from_token(token, flags, self, BracketType::Parenthesis)?;

                let wb = if matches!(next_token, Some((Token::Comma, _, _))) {
                    let wb_text = request_next_text(token_stream, &mut next_token, flags, self, &mut ending_index)?;
                    if wb_text == "wb" {
                        1
                    } else {
                        return Err(EncodingError::UnrecognizedOperand(flags.opcode, wb_text, None, self));
                    }
                } else {
                    0
                };

                utils::hex_num::u16_from_str(offset14).ok().and_then(|imm| {
                    regval::<Gpr>(reg_text, flags).map(|rs| {
                        let repaired = reshift_pair((EncodedFieldMask::r4000allegrex_offset14, (imm >> 2).into()), (EncodedFieldMask::rs, rs));

                        reshift_pair((EncodedFieldMask::r4000allegrex_wb, wb), (EncodedFieldMask::rs.union(EncodedFieldMask::r4000allegrex_offset14), repaired))
                    })
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                encode_r4000allegrex_vcmp_generic::<R4000AllegrexS>(
                    token,
                    token_stream,
                    &mut next_token,
                    flags,
                    self,
                    &mut ending_index,
                )?
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                encode_r4000allegrex_vcmp_generic::<R4000AllegrexV2D>(
                    token,
                    token_stream,
                    &mut next_token,
                    flags,
                    self,
                    &mut ending_index,
                )?
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                encode_r4000allegrex_vcmp_generic::<R4000AllegrexV3D>(
                    token,
                    token_stream,
                    &mut next_token,
                    flags,
                    self,
                    &mut ending_index,
                )?
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                encode_r4000allegrex_vcmp_generic::<R4000AllegrexV4D>(
                    token,
                    token_stream,
                    &mut next_token,
                    flags,
                    self,
                    &mut ending_index,
                )?
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vconstant => {
                regval_from_text_token::<R4000AllegrexVConstant>(token, flags, self)?
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_power_of_two => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vfpu_cc_bit => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_bn => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u8_from_str(text).ok().map(Into::into)
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_int16 => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::i16_from_str(text).ok().map(|x| (x as u16).into())
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_float16 => {
                let text = operand_text_from_token(token, flags, self)?;
                let float = text.parse::<f32>();
                float.ok().filter(|x| (-65504.0..=65504.0).contains(x) || x.is_nan() || x.is_infinite()).map(|x| {
                    utils::f16::repr_16_from_32(x.to_bits()).into()
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vrot_code => {
                let text = bracket_solo_from_token(token, flags, self, BracketType::Brackets)?;
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
                let text = bracket_solo_from_token(token, flags, self, BracketType::Brackets)?;
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
                let text = bracket_solo_from_token(token, flags, self, BracketType::Brackets)?;
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
            Self::r4000allegrex_wpx | Self::r4000allegrex_wpy | Self::r4000allegrex_wpz | Self::r4000allegrex_wpw => {
                // An empty operand is valid for some reason, so we have to
                // mess up with the token stream to allow this kind of stuff
                // i.e. `vpfxd       , , , ` is valid.
                (token, next_token) = if token == Token::Comma {
                    // Hacks to handle empty operands
                    if let Some(n) = next_token {
                        token_stream.push_front(n);
                    } else if self == Self::r4000allegrex_wpz {
                        token_stream.push_front((Token::Text(""), ending_index, ending_index));
                    }
                    (Token::Text(""), Some((Token::Comma, ending_index, ending_index+1)))
                } else {
                    // Hack to avoid erroring when wpz is not an empty string, but wpw is.
                    if self == Self::r4000allegrex_wpz {
                        if let Some(n) = token_stream.next_inner() {
                            // There's already something, so push it back in.
                            token_stream.push_front(n);
                        } else {
                            // There's nothing, so push an empty string to make wpw happy.
                            let index = next_token.as_ref().map_or(ending_index, |(_, _, end)| *end);
                            token_stream.push_front((Token::Text(""), index, index));
                        }
                    }
                    (token, next_token)
                };

                regval_from_text_token::<R4000AllegrexPrefixDst>(token, flags, self)?.map(|x| {
                    let c = x >> 2;
                    let d = x & utils::bitmask(0, 2);

                    let c = match self {
                        Self::r4000allegrex_wpx => c << (8),
                        Self::r4000allegrex_wpy => c << (9-2),
                        Self::r4000allegrex_wpz => c << (10-4),
                        Self::r4000allegrex_wpw => c << (11-6),
                        _ => unreachable!(),
                    };

                    c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpx => {
                regval_from_text_token::<R4000AllegrexPrefixSrc>(token, flags, self)?.map(|x| {
                    let a = ((x & utils::bitmask(4, 1)) >> 4) << 16;
                    let b = ((x & utils::bitmask(3, 1)) >> 3) << 12;
                    let c = ((x & utils::bitmask(2, 1)) >> 2) << 8;
                    let d = x & utils::bitmask(0, 2);

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpy => {
                regval_from_text_token::<R4000AllegrexPrefixSrc>(token, flags, self)?.map(|x| {
                    const SHIFT_VAL: u32 = 2;
                    let a = ((x & utils::bitmask(4, 1)) >> 4) << (17 - SHIFT_VAL);
                    let b = ((x & utils::bitmask(3, 1)) >> 3) << (13 - SHIFT_VAL);
                    let c = ((x & utils::bitmask(2, 1)) >> 2) << (9 - SHIFT_VAL);
                    let d = (x & utils::bitmask(0, 2)) << (2 - SHIFT_VAL);

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpz => {
                regval_from_text_token::<R4000AllegrexPrefixSrc>(token, flags, self)?.map(|x| {
                    const SHIFT_VAL: u32 = 4;
                    let a = ((x & utils::bitmask(4, 1)) >> 4) << (18 - SHIFT_VAL);
                    let b = ((x & utils::bitmask(3, 1)) >> 3) << (14 - SHIFT_VAL);
                    let c = ((x & utils::bitmask(2, 1)) >> 2) << (10 - SHIFT_VAL);
                    let d = (x & utils::bitmask(0, 2)) << (4 - SHIFT_VAL);

                    a | b | c | d
                })
            }
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpw => {
                regval_from_text_token::<R4000AllegrexPrefixSrc>(token, flags, self)?.map(|x| {
                    const SHIFT_VAL: u32 = 6;
                    let a = ((x & utils::bitmask(4, 1)) >> 4) << (19 - SHIFT_VAL);
                    let b = ((x & utils::bitmask(3, 1)) >> 3) << (15 - SHIFT_VAL);
                    let c = ((x & utils::bitmask(2, 1)) >> 2) << (11 - SHIFT_VAL);
                    let d = (x & utils::bitmask(0, 2)) << (6 - SHIFT_VAL);

                    a | b | c | d
                })
            }

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_I => {
                let text = operand_text_from_token(token, flags, self)?;
                matches!(text, "I" | "$I").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_Q => {
                let text = operand_text_from_token(token, flags, self)?;
                matches!(text, "Q" | "$Q").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_R => {
                let text = operand_text_from_token(token, flags, self)?;
                matches!(text, "R" | "$R").then_some(0)
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACC => None /*Self::r5900ee_ACC()*/,

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm5 => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::i8_from_str(text).ok().map(|x| (x as u8).into())
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm15 => {
                let text = operand_text_from_token(token, flags, self)?;
                utils::hex_num::u32_from_str(text).ok().map(|x| x >> 3)
            }

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfs | Self::r5900ee_vft | Self::r5900ee_vfd => {
                regval_from_text_token::<R5900EEVF>(token, flags, self)?
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis | Self::r5900ee_vit | Self::r5900ee_vid => {
                regval_from_text_token::<R5900EEVI>(token, flags, self)?
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACCxyzw => {
                let text = operand_text_from_token(token, flags, self)?;
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
                regval_from_text_token::<R5900EEVF>(token, flags, self)?
                /*
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
                */
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsxyzw_inv_vft => {
                // r5900ee_vfsxyzw_inv_vft has a generic mask that has both bitpatterns,
                // so we need to decide the specific mask to use here.
                mask = if flags.r5900ee_prodg_sn_as_inverted_regs() { EncodedFieldMask::r5900ee_vft} else { EncodedFieldMask::r5900ee_vfs};
                regval_from_text_token::<R5900EEVF>(token, flags, self)?
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftxyzw_inv_vfs => {
                // r5900ee_vftxyzw_inv_vfs has a generic mask that has both bitpatterns,
                // so we need to decide the specific mask to use here.
                mask = if flags.r5900ee_prodg_sn_as_inverted_regs() { EncodedFieldMask::r5900ee_vfs} else { EncodedFieldMask::r5900ee_vft};
                regval_from_text_token::<R5900EEVF>(token, flags, self)?
            }

            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftn => {
                let text = operand_text_from_token(token, flags, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let n = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], flags).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vft, reg), (EncodedFieldMask::r5900ee_n, n))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsl => {
                let text = operand_text_from_token(token, flags, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let l = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], flags).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vfs, reg), (EncodedFieldMask::r5900ee_l, l))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftm => {
                let text = operand_text_from_token(token, flags, self)?;
                if text.ends_with('x') || text.ends_with('y') || text.ends_with('z') || text.ends_with('w') {
                    let m = match text.chars().last().expect("Already checked the string is long enough") {
                        'x' => 0,
                        'y' => 1,
                        'z' => 2,
                        'w' => 3,
                        _ => unreachable!(),
                    };
                    regval::<R5900EEVF>(&text[..text.len()-1], flags).map(|reg| {
                        reshift_pair((EncodedFieldMask::r5900ee_vft, reg), (EncodedFieldMask::r5900ee_m, m))
                    })
                } else {
                    None
                }
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_predecr | Self::r5900ee_vit_predecr => {
                let text = bracket_solo_from_token(token, flags, self, BracketType::Parenthesis)?;
                text.strip_prefix("--").and_then(|reg| regval::<R5900EEVI>(reg, flags))
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_postincr | Self::r5900ee_vit_postincr => {
                let text = bracket_solo_from_token(token, flags, self, BracketType::Parenthesis)?;
                text.strip_suffix("++").and_then(|reg| regval::<R5900EEVI>(reg, flags))
            }
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_parenthesis => {
                let text = bracket_solo_from_token(token, flags, self, BracketType::Parenthesis)?;
                regval::<R5900EEVI>(text, flags)
            }
        };

        let encoded = if let Some(val) = val {
            mask.unshift(val)
        } else {
            let err = match token {
                Token::End => EncodingError::EndTokenInsteadOfOperand(flags.opcode, self),
                Token::Comma => EncodingError::CommaInsteadOfOperand(flags.opcode, self),
                Token::Text(text) | Token::DottedText(TokenDottedText { full: text, .. }) => {
                    EncodingError::UnrecognizedOperand(flags.opcode, text, None, self)
                }
                Token::Bracketed(left, right, bracket_type) => EncodingError::UnrecognizedOperand(
                    flags.opcode,
                    left,
                    Some((right, bracket_type)),
                    self,
                ),
                Token::BracketSolo(text, bracket_type) => EncodingError::UnrecognizedOperand(
                    flags.opcode,
                    "",
                    Some((text, bracket_type)),
                    self,
                ),
            };

            return Err(err);
        };

        EncodedOperandBits::new(
            encoded,
            next_token.map(|x| x.0),
            flags,
            self,
            starting_index,
            ending_index,
        )
    }
}

fn regval<R>(name: &str, flags: &OperandEncoderFlags) -> Option<u32>
where
    R: Register,
{
    R::from_name(name, flags.abi(), flags.allow_dollarless()).map(|x| x.as_index() as u32)
}

const fn operand_text_from_token<'s>(
    token: Token<'s>,
    flags: &OperandEncoderFlags,
    operand: Operand,
) -> Result<&'s str, EncodingError<'s>> {
    match token {
        Token::Text(text) | Token::DottedText(TokenDottedText { full: text, .. }) => Ok(text),

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(
            flags.opcode,
            operand,
        )),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(flags.opcode, operand)),
        Token::Bracketed(left, right, bracket_type) => {
            Err(EncodingError::BracketedInsteadOfSingleOperand(
                flags.opcode,
                operand,
                left,
                right,
                bracket_type,
            ))
        }
        Token::BracketSolo(text, bracket_type) => {
            Err(EncodingError::BracketSoloInsteadOfSingleOperand(
                flags.opcode,
                operand,
                text,
                bracket_type,
            ))
        }
    }
}

fn regval_from_text_token<'s, R>(
    token: Token<'s>,
    flags: &OperandEncoderFlags,
    operand: Operand,
) -> Result<Option<u32>, EncodingError<'s>>
where
    R: Register,
{
    let text = operand_text_from_token(token, flags, operand)?;
    Ok(regval::<R>(text, flags))
}

fn bracketed_text_from_token<'s>(
    token: Token<'s>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    required_bracket_type: BracketType,
) -> Result<(&'s str, &'s str), EncodingError<'s>> {
    match token {
        Token::Bracketed(left, right, bracket_type) => {
            if bracket_type == required_bracket_type {
                Ok((left, right))
            } else {
                Err(EncodingError::WrongBracketedOperand(
                    flags.opcode,
                    operand,
                    left,
                    right,
                    bracket_type,
                    required_bracket_type,
                ))
            }
        }

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(
            flags.opcode,
            operand,
        )),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(flags.opcode, operand)),
        Token::Text(text) | Token::DottedText(TokenDottedText { full: text, .. }) => {
            Err(EncodingError::TextInsteadOfBracketedOperand(
                flags.opcode,
                operand,
                text,
                required_bracket_type,
            ))
        }
        Token::BracketSolo(text, bracket_type) => {
            Err(EncodingError::BracketSoloInsteadOfBracketedOperand(
                flags.opcode,
                operand,
                text,
                bracket_type,
                required_bracket_type,
            ))
        }
    }
}

#[cfg(any(feature = "R4000ALLEGREX", feature = "R5900EE"))]
fn bracket_solo_from_token<'s>(
    token: Token<'s>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    required_bracket_type: BracketType,
) -> Result<&'s str, EncodingError<'s>> {
    match token {
        Token::BracketSolo(text, bracket_type) => {
            if bracket_type == required_bracket_type {
                Ok(text)
            } else {
                Err(EncodingError::WrongBracketedOperand(
                    flags.opcode,
                    operand,
                    "",
                    text,
                    bracket_type,
                    required_bracket_type,
                ))
            }
        }

        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(
            flags.opcode,
            operand,
        )),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(flags.opcode, operand)),
        Token::Text(text) | Token::DottedText(TokenDottedText { full: text, .. }) => {
            Err(EncodingError::TextInsteadOfBracketedOperand(
                flags.opcode,
                operand,
                text,
                required_bracket_type,
            ))
        }
        Token::Bracketed(left, right, bracket_type) => {
            Err(EncodingError::BracketedInsteadOfBracketSoloOperand(
                flags.opcode,
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
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    ending_index: &mut usize,
) -> Result<&'s str, EncodingError<'s>> {
    if !matches!(next_token, Some((Token::Comma, _, _))) {
        return Err(EncodingError::MissingCommaInComposedOperand(
            flags.opcode,
            operand,
        ));
    }

    let Some(((token_aux, _, end), next_token_aux)) = token_stream.next() else {
        return Err(EncodingError::RanOutOfTokens(flags.opcode, operand));
    };
    *next_token = next_token_aux;
    *ending_index = end;

    operand_text_from_token(token_aux, flags, operand)
}

fn request_next_reg<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    ending_index: &mut usize,
) -> Result<R, EncodingError<'s>>
where
    R: Register,
{
    let text = request_next_text(token_stream, next_token, flags, operand, ending_index)?;

    R::from_name(text, flags.abi(), flags.allow_dollarless()).ok_or(
        EncodingError::UnrecognizedOperand(flags.opcode, text, None, operand),
    )
}

fn encode_next_reg<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    ending_index: &mut usize,
) -> Result<u32, EncodingError<'s>>
where
    R: Register,
{
    let reg: R = request_next_reg(token_stream, next_token, flags, operand, ending_index)?;
    Ok(reg.as_index() as u32)
}

#[cfg(feature = "RSP")]
fn parse_rsp_element_hq<'s>(
    text: &'s str,
    flags: &OperandEncoderFlags,
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

    utils::hex_num::u8_from_str(element)
        .map_err(|_| EncodingError::UnrecognizedOperand(flags.opcode, text, None, operand))
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

#[cfg(feature = "RSP")]
fn encode_rsp_offset_rs<'s, T, F, E>(
    token: Token<'s>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    converter: F,
    shift_value: T,
) -> Result<Option<u32>, EncodingError<'s>>
where
    T: core::ops::Shr<Output = T> + Into<u32>,
    F: FnOnce(&str) -> Result<T, E>,
{
    let bracket_type = BracketType::Parenthesis;

    let (left, right) = bracketed_text_from_token(token, flags, operand, bracket_type)?;
    let offset: T = converter(left).map_err(|_| {
        EncodingError::UnrecognizedOperand(flags.opcode, left, Some((right, bracket_type)), operand)
    })? >> shift_value;
    let rs = regval::<Gpr>(right, flags).ok_or(EncodingError::UnrecognizedOperand(
        flags.opcode,
        left,
        Some((right, bracket_type)),
        operand,
    ))?;

    let val = reshift_pair(
        (EncodedFieldMask::rsp_offset, offset.into()),
        (EncodedFieldMask::rs, rs),
    );
    Ok(Some(val))
}

#[cfg(feature = "R4000ALLEGREX")]
fn identify_r4000allegrex_vcmp_registers<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    cond: R4000AllegrexVCond,
    ending_index: &mut usize,
) -> Result<(R, R), EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let (vs, vt) = match cond {
        R4000AllegrexVCond::fl | R4000AllegrexVCond::tr => {
            // The two arguments may be omitted if they are zero.
            if !matches!(next_token, Some((Token::Comma, _, _))) {
                (R::default(), R::default())
            } else {
                let vs = request_next_reg(token_stream, next_token, flags, operand, ending_index)?;
                let vt = if !matches!(next_token, Some((Token::Comma, _, _))) {
                    R::default()
                } else {
                    request_next_reg(token_stream, next_token, flags, operand, ending_index)?
                };
                (vs, vt)
            }
        }
        R4000AllegrexVCond::eq
        | R4000AllegrexVCond::lt
        | R4000AllegrexVCond::le
        | R4000AllegrexVCond::ne
        | R4000AllegrexVCond::ge
        | R4000AllegrexVCond::gt => {
            let vs = request_next_reg(token_stream, next_token, flags, operand, ending_index)?;
            let vt = request_next_reg(token_stream, next_token, flags, operand, ending_index)?;

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
            let vs = request_next_reg(token_stream, next_token, flags, operand, ending_index)?;

            /* The last register may be omitted if it is zero */
            let vt = if !matches!(next_token, Some((Token::Comma, _, _))) {
                R::default()
            } else {
                request_next_reg(token_stream, next_token, flags, operand, ending_index)?
            };

            (vs, vt)
        }
    };

    Ok((vs, vt))
}

#[cfg(feature = "R4000ALLEGREX")]
fn encode_r4000allegrex_vcmp_registers<'s, R>(
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    cond: R4000AllegrexVCond,
    ending_index: &mut usize,
) -> Result<u32, EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let (vs, vt) = identify_r4000allegrex_vcmp_registers::<R>(
        token_stream,
        next_token,
        flags,
        operand,
        cond,
        ending_index,
    )?;

    let cond_bits = EncodedFieldMask::r4000allegrex_vcmp_cond.unshift(cond.as_index() as u32);
    let vs_bits = EncodedFieldMask::r4000allegrex_vs.unshift(vs.as_index() as u32);
    let vt_bits = EncodedFieldMask::r4000allegrex_vt.unshift(vt.as_index() as u32);
    Ok(cond_bits | vs_bits | vt_bits)
}

#[cfg(feature = "R4000ALLEGREX")]
fn encode_r4000allegrex_vcmp_generic<'s, R>(
    token: Token<'s>,
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    next_token: &mut Option<(Token<'s>, usize, usize)>,
    flags: &OperandEncoderFlags,
    operand: Operand,
    ending_index: &mut usize,
) -> Result<Option<u32>, EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let text = operand_text_from_token(token, flags, operand)?;
    let cond = R4000AllegrexVCond::from_name(text, flags.abi(), flags.allow_dollarless());

    let ret = cond.map(|cond| {
        let bits = encode_r4000allegrex_vcmp_registers::<R>(
            token_stream,
            next_token,
            flags,
            operand,
            cond,
            ending_index,
        )?;
        Ok(bits)
    });

    ret.transpose()
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
    EndBits(u32, usize, usize),
    ContinueBits(u32, usize, usize),
}

impl EncodedOperandBits {
    const fn new<'s>(
        bits: u32,
        next_token: Option<Token<'s>>,
        flags: &OperandEncoderFlags,
        operand: Operand,
        starting_index: usize,
        ending_index: usize,
    ) -> Result<Self, EncodingError<'s>> {
        match next_token {
            None | Some(Token::End) => Ok(Self::EndBits(bits, starting_index, ending_index)),
            Some(Token::Comma) => Ok(Self::ContinueBits(bits, starting_index, ending_index)),

            Some(Token::Text(t) | Token::DottedText(TokenDottedText { full: t, .. })) => Err(
                EncodingError::TokenInsteadOfCommaEnd(flags.opcode, operand, t),
            ),
            Some(Token::Bracketed(left, right, bracket_type)) => {
                Err(EncodingError::BracketedInsteadOfCommaEnd(
                    flags.opcode,
                    operand,
                    left,
                    right,
                    bracket_type,
                ))
            }
            Some(Token::BracketSolo(text, bracket_type)) => {
                Err(EncodingError::BracketSoloInsteadOfCommaEnd(
                    flags.opcode,
                    operand,
                    text,
                    bracket_type,
                ))
            }
        }
    }
}
