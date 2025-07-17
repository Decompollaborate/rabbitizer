/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use crate::abi::Abi;
use crate::encoded_field_mask::EncodedFieldMask;
use crate::encoder::token::{Token, Tokenize};
use crate::encoder::EncodingError;
use crate::opcodes::Opcode;
use crate::operands::Operand;
use crate::registers::*;
#[cfg(feature = "R4000ALLEGREX")]
use crate::registers_meta::R4000AllegrexVectorRegister;
use crate::registers_meta::Register;
use crate::utils::{self, DoubleOptIterator};

impl Operand {
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
        let text = operand_text_from_token(token, opcode, self)?;

        let val = match self {
            Self::ALL_EMPTY => None,
            Self::core_rs | Self::core_rt | Self::core_rd => regval::<Gpr>(text, abi, allow_dollarless),
            Self::core_sa => utils::u8_hex_from_str(text).ok().map(|x| x.into()),
            Self::core_zero => None /*Self::core_zero()*/,
            Self::core_cop0d => regval::<Cop0>(text, abi, allow_dollarless),
            Self::core_cop0cd => regval::<Cop0Control>(text, abi, allow_dollarless),
            Self::core_fs | Self::core_ft | Self::core_fd => regval::<Cop1>(text, abi, allow_dollarless),
            Self::core_cop1cs => None /*Self::core_cop1cs(field.cop1cs_impl())*/,
            Self::core_cop2t | Self::core_cop2d => regval::<Cop2>(text, abi, allow_dollarless),
            Self::core_cop2cd => regval::<Cop2Control>(text, abi, allow_dollarless),
            Self::core_op => None /*Self::core_op(field.op_impl())*/,
            Self::core_hint => None /*Self::core_hint(field.hint_impl())*/,
            Self::core_code => None /*Self::core_code(
                field.code_upper_impl(),
                NonZeroU16::new(field.code_lower_impl()),
            )*/,
            Self::core_code_lower => None /*Self::core_code_lower(field.code_lower_impl())*/,
            Self::core_copraw => None /*Self::core_copraw(instr.word() & utils::bitmask(0, 25))*/,
            Self::core_label => None /*Self::core_label(instr.get_instr_index_as_vram_impl())*/,
            Self::core_imm_i16 => utils::i16_hex_from_str(text).ok().map(|x| (x as u16).into()),
            Self::core_imm_u16 => utils::u16_hex_from_str(text).ok().map(|x| x.into()),
            Self::core_branch_target_label => None /*{
                Self::core_branch_target_label(instr.get_branch_offset_impl())
            }*/,
            Self::core_imm_rs => None /*Self::core_imm_rs(field.imm_i16_impl(), field.rs_impl())*/,
            Self::core_maybe_rd_rs => None /*{
                let rd = field.rd_impl();
                let reg = if rd.holds_return_address(instr.abi()) {
                    None
                } else {
                    Some(rd)
                };
                Self::core_maybe_rd_rs(reg, field.rs_impl())
            }*/,
            Self::core_maybe_zero_rs => None /*Self::core_maybe_zero_rs((), field.rs_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_cop0d => None /*Self::rsp_cop0d(field.rsp_cop0d_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_cop2cd => None /*Self::rsp_cop2cd(field.rsp_cop2cd_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vs => None /*Self::rsp_vs(field.rsp_vs_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vd => None /*Self::rsp_vd(field.rsp_vd_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementhigh => None /*{
                Self::rsp_vt_elementhigh(field.rsp_vt_impl(), field.rsp_elementhigh_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vt_elementlow => None /*{
                Self::rsp_vt_elementlow(field.rsp_vt_impl(), field.rsp_elementlow_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vd_de => None /*Self::rsp_vd_de(field.rsp_vd_impl(), field.rsp_de_impl())*/,
            #[cfg(feature = "RSP")]
            Self::rsp_vs_index => None /*{
                Self::rsp_vs_index(field.rsp_vs_impl(), field.rsp_index_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_offset7_rs => None /*{
                Self::rsp_offset7_rs(field.rsp_offset7_impl(), field.rs_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_offset8_rs => None /*{
                Self::rsp_offset8_rs(field.rsp_offset8_impl(), field.rs_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_offset9_rs => None /*{
                Self::rsp_offset9_rs(field.rsp_offset9_impl(), field.rs_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_offset10_rs => None /*{
                Self::rsp_offset10_rs(field.rsp_offset10_impl(), field.rs_impl())
            }*/,
            #[cfg(feature = "RSP")]
            Self::rsp_offset11_rs => None /*{
                Self::rsp_offset11_rs(field.rsp_offset11_impl(), field.rs_impl())
            }*/,
            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_sf => None /*Self::r3000gte_sf(field.r3000gte_sf_impl())*/,
            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_mx => None /*Self::r3000gte_mx(field.r3000gte_mx_impl())*/,
            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_v => None /*Self::r3000gte_v(field.r3000gte_v_impl())*/,
            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_cv => None /*Self::r3000gte_cv(field.r3000gte_cv_impl())*/,
            #[cfg(feature = "R3000GTE")]
            Self::r3000gte_lm => None /*Self::r3000gte_lm(field.r3000gte_lm_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vs => None /*{
                Self::r4000allegrex_s_vs(field.r4000allegrex_s_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vt => None /*{
                Self::r4000allegrex_s_vt(field.r4000allegrex_s_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vd => None /*{
                Self::r4000allegrex_s_vd(field.r4000allegrex_s_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vt_imm => None /*{
                Self::r4000allegrex_s_vt_imm(field.r4000allegrex_s_vt_imm_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_s_vd_imm => None /*{
                Self::r4000allegrex_s_vd_imm(field.r4000allegrex_s_vd_imm_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vs => None /*{
                Self::r4000allegrex_p_vs(field.r4000allegrex_p_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vt => None /*{
                Self::r4000allegrex_p_vt(field.r4000allegrex_p_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vd => None /*{
                Self::r4000allegrex_p_vd(field.r4000allegrex_p_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vs => None /*{
                Self::r4000allegrex_t_vs(field.r4000allegrex_t_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vt => None /*{
                Self::r4000allegrex_t_vt(field.r4000allegrex_t_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vd => None /*{
                Self::r4000allegrex_t_vd(field.r4000allegrex_t_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vs => None /*{
                Self::r4000allegrex_q_vs(field.r4000allegrex_q_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vt => None /*{
                Self::r4000allegrex_q_vt(field.r4000allegrex_q_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vd => None /*{
                Self::r4000allegrex_q_vd(field.r4000allegrex_q_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vt_imm => None /*{
                Self::r4000allegrex_q_vt_imm(field.r4000allegrex_q_vt_imm_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs => None /*{
                Self::r4000allegrex_mp_vs(field.r4000allegrex_mp_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vt => None /*{
                Self::r4000allegrex_mp_vt(field.r4000allegrex_mp_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vd => None /*{
                Self::r4000allegrex_mp_vd(field.r4000allegrex_mp_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mp_vs_transpose => None /*{
                Self::r4000allegrex_mp_vs_transpose(field.r4000allegrex_mp_vs_transpose_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs => None /*{
                Self::r4000allegrex_mt_vs(field.r4000allegrex_mt_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vt => None /*{
                Self::r4000allegrex_mt_vt(field.r4000allegrex_mt_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vd => None /*{
                Self::r4000allegrex_mt_vd(field.r4000allegrex_mt_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mt_vs_transpose => None /*{
                Self::r4000allegrex_mt_vs_transpose(field.r4000allegrex_mt_vs_transpose_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs => None /*{
                Self::r4000allegrex_mq_vs(field.r4000allegrex_mq_vs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vt => None /*{
                Self::r4000allegrex_mq_vt(field.r4000allegrex_mq_vt_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vd => None /*{
                Self::r4000allegrex_mq_vd(field.r4000allegrex_mq_vd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_mq_vs_transpose => None /*{
                Self::r4000allegrex_mq_vs_transpose(field.r4000allegrex_mq_vs_transpose_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cs => None /*{
                Self::r4000allegrex_cop2cs(field.r4000allegrex_cop2cs_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_cop2cd => None /*{
                Self::r4000allegrex_cop2cd(field.r4000allegrex_cop2cd_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_pos => None /*Self::r4000allegrex_pos(field.r4000allegrex_pos_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size => None /*{
                Self::r4000allegrex_size(field.r4000allegrex_size_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_size_plus_pos => None /*{
                Self::r4000allegrex_size_plus_pos(field.r4000allegrex_size_plus_pos_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_imm3 => None /*{
                Self::r4000allegrex_imm3(field.r4000allegrex_imm3_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_base => None /*Self::r4000allegrex_offset14_base(
                field.r4000allegrex_offset14_impl(),
                field.rs_impl(),
            )*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_offset14_base_maybe_wb => None /*{
                Self::r4000allegrex_offset14_base_maybe_wb(
                    field.r4000allegrex_offset14_impl(),
                    field.rs_impl(),
                    field.r4000allegrex_wb_impl(),
                )
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_s_maybe_vs_maybe_vt => {
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexS>(
                            cond,
                            &mut next_token,
                            token_stream,
                            abi,
                            allow_dollarless,
                            opcode,
                            self,)?
                        )
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_p_maybe_vs_maybe_vt => {
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV2D>(
                            cond,
                            &mut next_token,
                            token_stream,
                            abi,
                            allow_dollarless,
                            opcode,
                            self,)?
                        )
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_t_maybe_vs_maybe_vt => {
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV3D>(
                            cond,
                            &mut next_token,
                            token_stream,
                            abi,
                            allow_dollarless,
                            opcode,
                            self,)?
                        )
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vcmp_cond_q_maybe_vs_maybe_vt => {
                match R4000AllegrexVCond::from_name(text, abi, allow_dollarless) {
                    None => None,
                    Some(cond) => {
                        Some(encode_r4000allegrex_vcmp_registers::<R4000AllegrexV4D>(
                            cond,
                            &mut next_token,
                            token_stream,
                            abi,
                            allow_dollarless,
                            opcode,
                            self,)?
                        )
                    }
                }
            },
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vconstant => None /*{
                Self::r4000allegrex_vconstant(field.r4000allegrex_vconstant_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_power_of_two => None /*{
                Self::r4000allegrex_power_of_two(field.r4000allegrex_power_of_two_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_vfpu_cc_bit => None /*{
                Self::r4000allegrex_vfpu_cc_bit(field.r4000allegrex_vfpu_cc_bit_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_bn => None /*Self::r4000allegrex_bn(field.r4000allegrex_bn_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_int16 => None /*{
                Self::r4000allegrex_int16(field.r4000allegrex_int16_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_float16 => None /*Self::r4000allegrex_float16(
                ordered_float::OrderedFloat(field.r4000allegrex_float16_impl()),
            )*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_p_vrot_code => None /*{
                Self::r4000allegrex_p_vrot_code(field.r4000allegrex_vrot_code_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_t_vrot_code => None /*{
                Self::r4000allegrex_t_vrot_code(field.r4000allegrex_vrot_code_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_q_vrot_code => None /*{
                Self::r4000allegrex_q_vrot_code(field.r4000allegrex_vrot_code_impl())
            }*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpx => None /*Self::r4000allegrex_wpx(field.r4000allegrex_wpx_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpy => None /*Self::r4000allegrex_wpy(field.r4000allegrex_wpy_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpz => None /*Self::r4000allegrex_wpz(field.r4000allegrex_wpz_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_wpw => None /*Self::r4000allegrex_wpw(field.r4000allegrex_wpw_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpx => None /*Self::r4000allegrex_rpx(field.r4000allegrex_rpx_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpy => None /*Self::r4000allegrex_rpy(field.r4000allegrex_rpy_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpz => None /*Self::r4000allegrex_rpz(field.r4000allegrex_rpz_impl())*/,
            #[cfg(feature = "R4000ALLEGREX")]
            Self::r4000allegrex_rpw => None /*Self::r4000allegrex_rpw(field.r4000allegrex_rpw_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_I => None /*Self::r5900ee_I()*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_Q => None /*Self::r5900ee_Q()*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_R => None /*Self::r5900ee_R()*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACC => None /*Self::r5900ee_ACC()*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm5 => None /*Self::r5900ee_imm5(field.r5900ee_imm5_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_imm15 => None /*Self::r5900ee_imm15(field.r5900ee_imm15_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfs => None /*Self::r5900ee_vfs(field.r5900ee_vfs_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vft => None /*Self::r5900ee_vft(field.r5900ee_vft_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfd => None /*Self::r5900ee_vfd(field.r5900ee_vfd_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis => None /*Self::r5900ee_vis(field.r5900ee_vis_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vit => None /*Self::r5900ee_vit(field.r5900ee_vit_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vid => None /*Self::r5900ee_vid(field.r5900ee_vid_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_ACCxyzw => None /*Self::r5900ee_ACCxyzw(
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            )*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsxyzw => None /*Self::r5900ee_vfsxyzw(
                field.r5900ee_vfs_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            )*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftxyzw => None /*Self::r5900ee_vftxyzw(
                field.r5900ee_vft_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            )*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfdxyzw => None /*Self::r5900ee_vfdxyzw(
                field.r5900ee_vfd_impl(),
                field.r5900ee_xyzw_x_impl(),
                field.r5900ee_xyzw_y_impl(),
                field.r5900ee_xyzw_z_impl(),
                field.r5900ee_xyzw_w_impl(),
            )*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftn => None /*{
                Self::r5900ee_vftn(field.r5900ee_vft_impl(), field.r5900ee_n_impl())
            }*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vfsl => None /*{
                Self::r5900ee_vfsl(field.r5900ee_vfs_impl(), field.r5900ee_l_impl())
            }*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vftm => None /*{
                Self::r5900ee_vftm(field.r5900ee_vft_impl(), field.r5900ee_m_impl())
            }*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_predecr => None /*Self::r5900ee_vis_predecr((), field.r5900ee_vis_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vit_predecr => None /*Self::r5900ee_vit_predecr((), field.r5900ee_vit_impl())*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_postincr => None /*{
                Self::r5900ee_vis_postincr(field.r5900ee_vis_impl(), ())
            }*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vit_postincr => None /*{
                Self::r5900ee_vit_postincr(field.r5900ee_vit_impl(), ())
            }*/,
            #[cfg(feature = "R5900EE")]
            Self::r5900ee_vis_parenthesis => None /*{
                Self::r5900ee_vis_parenthesis(field.r5900ee_vis_impl())
            }*/,
        };

        let encoded = if let Some(val) = val {
            self.mask().unshift(val)
        } else {
            return Err(EncodingError::UnrecognizedOperand(opcode, text, self));
        };

        EncodedOperandBits::new(encoded, next_token, opcode)
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
        Token::End => Err(EncodingError::EndTokenInsteadOfOperand(opcode, operand)),
        Token::Comma => Err(EncodingError::CommaInsteadOfOperand(opcode, operand)),
        Token::Text(text) => Ok(text),
    }
}

#[cfg(feature = "R4000ALLEGREX")]
fn identify_r4000allegrex_vcmp_registers<'s, R>(
    cond: R4000AllegrexVCond,
    next_token: &mut Option<Token<'s>>,
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    abi: Abi,
    allow_dollarless: bool,
    opcode: Opcode,
    operand: Operand,
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
                let Some((token_a, next_token_a)) = token_stream.next() else {
                    return Err(EncodingError::RanOutOfTokens(opcode, operand));
                };
                let text_a = operand_text_from_token(token_a, opcode, operand)?;
                let Some(reg_a) = R::from_name(text_a, abi, allow_dollarless) else {
                    return Err(EncodingError::UnrecognizedOperand(opcode, text_a, operand));
                };
                if next_token_a != Some(Token::Comma) {
                    return Err(EncodingError::MissingCommaInComposedOperand(
                        opcode, operand,
                    ));
                }

                let Some((token_b, next_token_b)) = token_stream.next() else {
                    return Err(EncodingError::RanOutOfTokens(opcode, operand));
                };
                let text_b = operand_text_from_token(token_b, opcode, operand)?;
                let Some(reg_b) = R::from_name(text_b, abi, allow_dollarless) else {
                    return Err(EncodingError::UnrecognizedOperand(opcode, text_b, operand));
                };

                *next_token = next_token_b;

                (reg_a, reg_b)
            }
        }
        R4000AllegrexVCond::eq
        | R4000AllegrexVCond::lt
        | R4000AllegrexVCond::le
        | R4000AllegrexVCond::ne
        | R4000AllegrexVCond::ge
        | R4000AllegrexVCond::gt => {
            if next_token != &Some(Token::Comma) {
                return Err(EncodingError::MissingCommaInComposedOperand(
                    opcode, operand,
                ));
            }

            let Some((token_a, next_token_a)) = token_stream.next() else {
                return Err(EncodingError::RanOutOfTokens(opcode, operand));
            };
            let text_a = operand_text_from_token(token_a, opcode, operand)?;
            let Some(reg_a) = R::from_name(text_a, abi, allow_dollarless) else {
                return Err(EncodingError::UnrecognizedOperand(opcode, text_a, operand));
            };
            if next_token_a != Some(Token::Comma) {
                return Err(EncodingError::MissingCommaInComposedOperand(
                    opcode, operand,
                ));
            }

            let Some((token_b, next_token_b)) = token_stream.next() else {
                return Err(EncodingError::RanOutOfTokens(opcode, operand));
            };
            let text_b = operand_text_from_token(token_b, opcode, operand)?;
            let Some(reg_b) = R::from_name(text_b, abi, allow_dollarless) else {
                return Err(EncodingError::UnrecognizedOperand(opcode, text_b, operand));
            };

            *next_token = next_token_b;

            (reg_a, reg_b)
        }
        R4000AllegrexVCond::ez
        | R4000AllegrexVCond::en
        | R4000AllegrexVCond::ei
        | R4000AllegrexVCond::es
        | R4000AllegrexVCond::nz
        | R4000AllegrexVCond::nn
        | R4000AllegrexVCond::ni
        | R4000AllegrexVCond::ns => {
            if next_token != &Some(Token::Comma) {
                return Err(EncodingError::MissingCommaInComposedOperand(
                    opcode, operand,
                ));
            }

            let Some((token_a, next_token_a)) = token_stream.next() else {
                return Err(EncodingError::RanOutOfTokens(opcode, operand));
            };
            let text_a = operand_text_from_token(token_a, opcode, operand)?;
            let Some(reg_a) = R::from_name(text_a, abi, allow_dollarless) else {
                return Err(EncodingError::UnrecognizedOperand(opcode, text_a, operand));
            };

            /* The last register may be omitted if it is zero */
            let vt = if next_token_a != Some(Token::Comma) {
                *next_token = next_token_a;

                R::default()
            } else {
                let Some((token_b, next_token_b)) = token_stream.next() else {
                    return Err(EncodingError::RanOutOfTokens(opcode, operand));
                };
                let text_b = operand_text_from_token(token_b, opcode, operand)?;
                let Some(reg_b) = R::from_name(text_b, abi, allow_dollarless) else {
                    return Err(EncodingError::UnrecognizedOperand(opcode, text_b, operand));
                };

                *next_token = next_token_b;

                reg_b
            };

            (reg_a, vt)
        }
    };

    Ok((vs, vt))
}

#[cfg(feature = "R4000ALLEGREX")]
fn encode_r4000allegrex_vcmp_registers<'s, R>(
    cond: R4000AllegrexVCond,
    next_token: &mut Option<Token<'s>>,
    token_stream: &mut DoubleOptIterator<&mut Tokenize<'s>>,
    abi: Abi,
    allow_dollarless: bool,
    opcode: Opcode,
    operand: Operand,
) -> Result<u32, EncodingError<'s>>
where
    R: R4000AllegrexVectorRegister,
{
    let (vs, vt) = identify_r4000allegrex_vcmp_registers::<R>(
        cond,
        next_token,
        token_stream,
        abi,
        allow_dollarless,
        opcode,
        operand,
    )?;

    let cond_bits = EncodedFieldMask::r4000allegrex_vcmp_cond.unshift(cond.as_index() as u32);
    let vs_bits = EncodedFieldMask::r4000allegrex_vs.unshift(vs.as_index() as u32);
    let vt_bits = EncodedFieldMask::r4000allegrex_vt.unshift(vt.as_index() as u32);
    Ok(cond_bits | vs_bits | vt_bits)
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
    ) -> Result<Self, EncodingError<'s>> {
        match next_token {
            None | Some(Token::End) => Ok(Self::EndBits(bits)),
            Some(Token::Text(t)) => Err(EncodingError::TokenInsteadOfComma(opcode, t)),
            Some(Token::Comma) => Ok(Self::ContinueBits(bits)),
        }
    }
}
