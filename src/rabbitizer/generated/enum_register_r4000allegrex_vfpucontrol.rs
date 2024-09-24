/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::RegisterDescriptor;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexVfpuControl {
    VFPU_PFXS = 128,
    VFPU_PFXT = 129,
    VFPU_PFXD = 130,
    VFPU_CC = 131,
    VFPU_INF4 = 132,
    VFPU_RSV5 = 133,
    VFPU_RSV6 = 134,
    VFPU_REV = 135,
    VFPU_RCX0 = 136,
    VFPU_RCX1 = 137,
    VFPU_RCX2 = 138,
    VFPU_RCX3 = 139,
    VFPU_RCX4 = 140,
    VFPU_RCX5 = 141,
    VFPU_RCX6 = 142,
    VFPU_RCX7 = 143,
    r144 = 144,
    r145 = 145,
    r146 = 146,
    r147 = 147,
    r148 = 148,
    r149 = 149,
    r150 = 150,
    r151 = 151,
    r152 = 152,
    r153 = 153,
    r154 = 154,
    r155 = 155,
    r156 = 156,
    r157 = 157,
    r158 = 158,
    r159 = 159,
    r160 = 160,
    r161 = 161,
    r162 = 162,
    r163 = 163,
    r164 = 164,
    r165 = 165,
    r166 = 166,
    r167 = 167,
    r168 = 168,
    r169 = 169,
    r170 = 170,
    r171 = 171,
    r172 = 172,
    r173 = 173,
    r174 = 174,
    r175 = 175,
    r176 = 176,
    r177 = 177,
    r178 = 178,
    r179 = 179,
    r180 = 180,
    r181 = 181,
    r182 = 182,
    r183 = 183,
    r184 = 184,
    r185 = 185,
    r186 = 186,
    r187 = 187,
    r188 = 188,
    r189 = 189,
    r190 = 190,
    r191 = 191,
    r192 = 192,
    r193 = 193,
    r194 = 194,
    r195 = 195,
    r196 = 196,
    r197 = 197,
    r198 = 198,
    r199 = 199,
    r200 = 200,
    r201 = 201,
    r202 = 202,
    r203 = 203,
    r204 = 204,
    r205 = 205,
    r206 = 206,
    r207 = 207,
    r208 = 208,
    r209 = 209,
    r210 = 210,
    r211 = 211,
    r212 = 212,
    r213 = 213,
    r214 = 214,
    r215 = 215,
    r216 = 216,
    r217 = 217,
    r218 = 218,
    r219 = 219,
    r220 = 220,
    r221 = 221,
    r222 = 222,
    r223 = 223,
    r224 = 224,
    r225 = 225,
    r226 = 226,
    r227 = 227,
    r228 = 228,
    r229 = 229,
    r230 = 230,
    r231 = 231,
    r232 = 232,
    r233 = 233,
    r234 = 234,
    r235 = 235,
    r236 = 236,
    r237 = 237,
    r238 = 238,
    r239 = 239,
    r240 = 240,
    r241 = 241,
    r242 = 242,
    r243 = 243,
    r244 = 244,
    r245 = 245,
    r246 = 246,
    r247 = 247,
    r248 = 248,
    r249 = 249,
    r250 = 250,
    r251 = 251,
    r252 = 252,
    r253 = 253,
    r254 = 254,
    r255 = 255,
}
pub static R4000ALLEGREX_VFPUCONTROL: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexVfpuControl::VFPU_PFXS as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXS", 128, concat!("$", "128"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_PFXT as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXT", 129, concat!("$", "129"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_PFXD as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXD", 130, concat!("$", "130"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_CC as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_CC", 131, concat!("$", "131"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_INF4 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_INF4", 132, concat!("$", "132"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RSV5 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV5", 133, concat!("$", "133"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RSV6 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV6", 134, concat!("$", "134"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_REV as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_REV", 135, concat!("$", "135"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX0 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX0", 136, concat!("$", "136"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX1 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX1", 137, concat!("$", "137"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX2 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX2", 138, concat!("$", "138"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX3 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX3", 139, concat!("$", "139"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX4 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX4", 140, concat!("$", "140"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX5 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX5", 141, concat!("$", "141"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX6 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX6", 142, concat!("$", "142"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::VFPU_RCX7 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX7", 143, concat!("$", "143"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r144 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "144"), 144, concat!("$", "144"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r145 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "145"), 145, concat!("$", "145"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r146 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "146"), 146, concat!("$", "146"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r147 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "147"), 147, concat!("$", "147"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r148 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "148"), 148, concat!("$", "148"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r149 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "149"), 149, concat!("$", "149"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r150 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "150"), 150, concat!("$", "150"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r151 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "151"), 151, concat!("$", "151"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r152 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "152"), 152, concat!("$", "152"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r153 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "153"), 153, concat!("$", "153"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r154 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "154"), 154, concat!("$", "154"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r155 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "155"), 155, concat!("$", "155"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r156 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "156"), 156, concat!("$", "156"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r157 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "157"), 157, concat!("$", "157"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r158 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "158"), 158, concat!("$", "158"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r159 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "159"), 159, concat!("$", "159"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r160 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "160"), 160, concat!("$", "160"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r161 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "161"), 161, concat!("$", "161"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r162 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "162"), 162, concat!("$", "162"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r163 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "163"), 163, concat!("$", "163"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r164 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "164"), 164, concat!("$", "164"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r165 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "165"), 165, concat!("$", "165"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r166 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "166"), 166, concat!("$", "166"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r167 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "167"), 167, concat!("$", "167"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r168 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "168"), 168, concat!("$", "168"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r169 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "169"), 169, concat!("$", "169"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r170 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "170"), 170, concat!("$", "170"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r171 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "171"), 171, concat!("$", "171"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r172 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "172"), 172, concat!("$", "172"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r173 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "173"), 173, concat!("$", "173"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r174 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "174"), 174, concat!("$", "174"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r175 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "175"), 175, concat!("$", "175"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r176 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "176"), 176, concat!("$", "176"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r177 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "177"), 177, concat!("$", "177"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r178 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "178"), 178, concat!("$", "178"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r179 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "179"), 179, concat!("$", "179"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r180 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "180"), 180, concat!("$", "180"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r181 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "181"), 181, concat!("$", "181"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r182 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "182"), 182, concat!("$", "182"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r183 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "183"), 183, concat!("$", "183"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r184 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "184"), 184, concat!("$", "184"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r185 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "185"), 185, concat!("$", "185"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r186 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "186"), 186, concat!("$", "186"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r187 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "187"), 187, concat!("$", "187"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r188 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "188"), 188, concat!("$", "188"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r189 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "189"), 189, concat!("$", "189"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r190 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "190"), 190, concat!("$", "190"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r191 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "191"), 191, concat!("$", "191"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r192 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "192"), 192, concat!("$", "192"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r193 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "193"), 193, concat!("$", "193"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r194 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "194"), 194, concat!("$", "194"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r195 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "195"), 195, concat!("$", "195"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r196 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "196"), 196, concat!("$", "196"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r197 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "197"), 197, concat!("$", "197"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r198 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "198"), 198, concat!("$", "198"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r199 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "199"), 199, concat!("$", "199"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r200 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "200"), 200, concat!("$", "200"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r201 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "201"), 201, concat!("$", "201"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r202 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "202"), 202, concat!("$", "202"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r203 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "203"), 203, concat!("$", "203"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r204 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "204"), 204, concat!("$", "204"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r205 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "205"), 205, concat!("$", "205"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r206 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "206"), 206, concat!("$", "206"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r207 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "207"), 207, concat!("$", "207"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r208 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "208"), 208, concat!("$", "208"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r209 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "209"), 209, concat!("$", "209"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r210 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "210"), 210, concat!("$", "210"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r211 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "211"), 211, concat!("$", "211"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r212 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "212"), 212, concat!("$", "212"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r213 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "213"), 213, concat!("$", "213"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r214 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "214"), 214, concat!("$", "214"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r215 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "215"), 215, concat!("$", "215"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r216 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "216"), 216, concat!("$", "216"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r217 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "217"), 217, concat!("$", "217"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r218 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "218"), 218, concat!("$", "218"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r219 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "219"), 219, concat!("$", "219"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r220 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "220"), 220, concat!("$", "220"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r221 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "221"), 221, concat!("$", "221"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r222 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "222"), 222, concat!("$", "222"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r223 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "223"), 223, concat!("$", "223"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r224 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "224"), 224, concat!("$", "224"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r225 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "225"), 225, concat!("$", "225"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r226 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "226"), 226, concat!("$", "226"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r227 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "227"), 227, concat!("$", "227"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r228 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "228"), 228, concat!("$", "228"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r229 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "229"), 229, concat!("$", "229"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r230 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "230"), 230, concat!("$", "230"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r231 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "231"), 231, concat!("$", "231"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r232 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "232"), 232, concat!("$", "232"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r233 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "233"), 233, concat!("$", "233"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r234 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "234"), 234, concat!("$", "234"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r235 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "235"), 235, concat!("$", "235"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r236 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "236"), 236, concat!("$", "236"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r237 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "237"), 237, concat!("$", "237"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r238 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "238"), 238, concat!("$", "238"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r239 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "239"), 239, concat!("$", "239"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r240 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "240"), 240, concat!("$", "240"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r241 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "241"), 241, concat!("$", "241"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r242 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "242"), 242, concat!("$", "242"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r243 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "243"), 243, concat!("$", "243"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r244 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "244"), 244, concat!("$", "244"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r245 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "245"), 245, concat!("$", "245"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r246 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "246"), 246, concat!("$", "246"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r247 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "247"), 247, concat!("$", "247"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r248 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "248"), 248, concat!("$", "248"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r249 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "249"), 249, concat!("$", "249"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r250 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "250"), 250, concat!("$", "250"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r251 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "251"), 251, concat!("$", "251"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r252 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "252"), 252, concat!("$", "252"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r253 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "253"), 253, concat!("$", "253"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r254 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "254"), 254, concat!("$", "254"))
    }
    .check_panic_chain();
    table[R4000AllegrexVfpuControl::r255 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "255"), 255, concat!("$", "255"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize - 128 == i, "Broken register index?");
        i += 1;
    }
    table
};
impl R4000AllegrexVfpuControl {
    pub const fn try_from_u32(value: u32) -> Result<Self, crate::Error> {
        match value {
            128 => Ok(Self::VFPU_PFXS),
            129 => Ok(Self::VFPU_PFXT),
            130 => Ok(Self::VFPU_PFXD),
            131 => Ok(Self::VFPU_CC),
            132 => Ok(Self::VFPU_INF4),
            133 => Ok(Self::VFPU_RSV5),
            134 => Ok(Self::VFPU_RSV6),
            135 => Ok(Self::VFPU_REV),
            136 => Ok(Self::VFPU_RCX0),
            137 => Ok(Self::VFPU_RCX1),
            138 => Ok(Self::VFPU_RCX2),
            139 => Ok(Self::VFPU_RCX3),
            140 => Ok(Self::VFPU_RCX4),
            141 => Ok(Self::VFPU_RCX5),
            142 => Ok(Self::VFPU_RCX6),
            143 => Ok(Self::VFPU_RCX7),
            144 => Ok(Self::r144),
            145 => Ok(Self::r145),
            146 => Ok(Self::r146),
            147 => Ok(Self::r147),
            148 => Ok(Self::r148),
            149 => Ok(Self::r149),
            150 => Ok(Self::r150),
            151 => Ok(Self::r151),
            152 => Ok(Self::r152),
            153 => Ok(Self::r153),
            154 => Ok(Self::r154),
            155 => Ok(Self::r155),
            156 => Ok(Self::r156),
            157 => Ok(Self::r157),
            158 => Ok(Self::r158),
            159 => Ok(Self::r159),
            160 => Ok(Self::r160),
            161 => Ok(Self::r161),
            162 => Ok(Self::r162),
            163 => Ok(Self::r163),
            164 => Ok(Self::r164),
            165 => Ok(Self::r165),
            166 => Ok(Self::r166),
            167 => Ok(Self::r167),
            168 => Ok(Self::r168),
            169 => Ok(Self::r169),
            170 => Ok(Self::r170),
            171 => Ok(Self::r171),
            172 => Ok(Self::r172),
            173 => Ok(Self::r173),
            174 => Ok(Self::r174),
            175 => Ok(Self::r175),
            176 => Ok(Self::r176),
            177 => Ok(Self::r177),
            178 => Ok(Self::r178),
            179 => Ok(Self::r179),
            180 => Ok(Self::r180),
            181 => Ok(Self::r181),
            182 => Ok(Self::r182),
            183 => Ok(Self::r183),
            184 => Ok(Self::r184),
            185 => Ok(Self::r185),
            186 => Ok(Self::r186),
            187 => Ok(Self::r187),
            188 => Ok(Self::r188),
            189 => Ok(Self::r189),
            190 => Ok(Self::r190),
            191 => Ok(Self::r191),
            192 => Ok(Self::r192),
            193 => Ok(Self::r193),
            194 => Ok(Self::r194),
            195 => Ok(Self::r195),
            196 => Ok(Self::r196),
            197 => Ok(Self::r197),
            198 => Ok(Self::r198),
            199 => Ok(Self::r199),
            200 => Ok(Self::r200),
            201 => Ok(Self::r201),
            202 => Ok(Self::r202),
            203 => Ok(Self::r203),
            204 => Ok(Self::r204),
            205 => Ok(Self::r205),
            206 => Ok(Self::r206),
            207 => Ok(Self::r207),
            208 => Ok(Self::r208),
            209 => Ok(Self::r209),
            210 => Ok(Self::r210),
            211 => Ok(Self::r211),
            212 => Ok(Self::r212),
            213 => Ok(Self::r213),
            214 => Ok(Self::r214),
            215 => Ok(Self::r215),
            216 => Ok(Self::r216),
            217 => Ok(Self::r217),
            218 => Ok(Self::r218),
            219 => Ok(Self::r219),
            220 => Ok(Self::r220),
            221 => Ok(Self::r221),
            222 => Ok(Self::r222),
            223 => Ok(Self::r223),
            224 => Ok(Self::r224),
            225 => Ok(Self::r225),
            226 => Ok(Self::r226),
            227 => Ok(Self::r227),
            228 => Ok(Self::r228),
            229 => Ok(Self::r229),
            230 => Ok(Self::r230),
            231 => Ok(Self::r231),
            232 => Ok(Self::r232),
            233 => Ok(Self::r233),
            234 => Ok(Self::r234),
            235 => Ok(Self::r235),
            236 => Ok(Self::r236),
            237 => Ok(Self::r237),
            238 => Ok(Self::r238),
            239 => Ok(Self::r239),
            240 => Ok(Self::r240),
            241 => Ok(Self::r241),
            242 => Ok(Self::r242),
            243 => Ok(Self::r243),
            244 => Ok(Self::r244),
            245 => Ok(Self::r245),
            246 => Ok(Self::r246),
            247 => Ok(Self::r247),
            248 => Ok(Self::r248),
            249 => Ok(Self::r249),
            250 => Ok(Self::r250),
            251 => Ok(Self::r251),
            252 => Ok(Self::r252),
            253 => Ok(Self::r253),
            254 => Ok(Self::r254),
            255 => Ok(Self::r255),
            x => Err(crate::Error::OutOfRangeRegisterIndex {
                index: x,
                count: 128,
                register_kind: "R4000AllegrexVfpuControl",
            }),
        }
    }
}
impl TryFrom<u32> for R4000AllegrexVfpuControl {
    type Error = crate::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from_u32(value)
    }
}
impl Default for R4000AllegrexVfpuControl {
    fn default() -> Self {
        Self::default()
    }
}
impl Index<R4000AllegrexVfpuControl> for [RegisterDescriptor] {
    type Output = RegisterDescriptor;
    fn index(&self, index: R4000AllegrexVfpuControl) -> &Self::Output {
        &self[index as usize]
    }
}
