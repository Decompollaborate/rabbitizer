/* SPDX-FileCopyrightText: © 2022-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::register_descriptors::RegisterDescriptor;
use crate::registers_meta::IntRegisterConversionError;
use core::ops::Index;
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[allow(clippy::exhaustive_enums)]
pub enum R4000AllegrexVfpuControl {
    VFPU_PFXS = 128 - 128,
    VFPU_PFXT = 129 - 128,
    VFPU_PFXD = 130 - 128,
    VFPU_CC = 131 - 128,
    VFPU_INF4 = 132 - 128,
    VFPU_RSV5 = 133 - 128,
    VFPU_RSV6 = 134 - 128,
    VFPU_REV = 135 - 128,
    VFPU_RCX0 = 136 - 128,
    VFPU_RCX1 = 137 - 128,
    VFPU_RCX2 = 138 - 128,
    VFPU_RCX3 = 139 - 128,
    VFPU_RCX4 = 140 - 128,
    VFPU_RCX5 = 141 - 128,
    VFPU_RCX6 = 142 - 128,
    VFPU_RCX7 = 143 - 128,
    r144 = 144 - 128,
    r145 = 145 - 128,
    r146 = 146 - 128,
    r147 = 147 - 128,
    r148 = 148 - 128,
    r149 = 149 - 128,
    r150 = 150 - 128,
    r151 = 151 - 128,
    r152 = 152 - 128,
    r153 = 153 - 128,
    r154 = 154 - 128,
    r155 = 155 - 128,
    r156 = 156 - 128,
    r157 = 157 - 128,
    r158 = 158 - 128,
    r159 = 159 - 128,
    r160 = 160 - 128,
    r161 = 161 - 128,
    r162 = 162 - 128,
    r163 = 163 - 128,
    r164 = 164 - 128,
    r165 = 165 - 128,
    r166 = 166 - 128,
    r167 = 167 - 128,
    r168 = 168 - 128,
    r169 = 169 - 128,
    r170 = 170 - 128,
    r171 = 171 - 128,
    r172 = 172 - 128,
    r173 = 173 - 128,
    r174 = 174 - 128,
    r175 = 175 - 128,
    r176 = 176 - 128,
    r177 = 177 - 128,
    r178 = 178 - 128,
    r179 = 179 - 128,
    r180 = 180 - 128,
    r181 = 181 - 128,
    r182 = 182 - 128,
    r183 = 183 - 128,
    r184 = 184 - 128,
    r185 = 185 - 128,
    r186 = 186 - 128,
    r187 = 187 - 128,
    r188 = 188 - 128,
    r189 = 189 - 128,
    r190 = 190 - 128,
    r191 = 191 - 128,
    r192 = 192 - 128,
    r193 = 193 - 128,
    r194 = 194 - 128,
    r195 = 195 - 128,
    r196 = 196 - 128,
    r197 = 197 - 128,
    r198 = 198 - 128,
    r199 = 199 - 128,
    r200 = 200 - 128,
    r201 = 201 - 128,
    r202 = 202 - 128,
    r203 = 203 - 128,
    r204 = 204 - 128,
    r205 = 205 - 128,
    r206 = 206 - 128,
    r207 = 207 - 128,
    r208 = 208 - 128,
    r209 = 209 - 128,
    r210 = 210 - 128,
    r211 = 211 - 128,
    r212 = 212 - 128,
    r213 = 213 - 128,
    r214 = 214 - 128,
    r215 = 215 - 128,
    r216 = 216 - 128,
    r217 = 217 - 128,
    r218 = 218 - 128,
    r219 = 219 - 128,
    r220 = 220 - 128,
    r221 = 221 - 128,
    r222 = 222 - 128,
    r223 = 223 - 128,
    r224 = 224 - 128,
    r225 = 225 - 128,
    r226 = 226 - 128,
    r227 = 227 - 128,
    r228 = 228 - 128,
    r229 = 229 - 128,
    r230 = 230 - 128,
    r231 = 231 - 128,
    r232 = 232 - 128,
    r233 = 233 - 128,
    r234 = 234 - 128,
    r235 = 235 - 128,
    r236 = 236 - 128,
    r237 = 237 - 128,
    r238 = 238 - 128,
    r239 = 239 - 128,
    r240 = 240 - 128,
    r241 = 241 - 128,
    r242 = 242 - 128,
    r243 = 243 - 128,
    r244 = 244 - 128,
    r245 = 245 - 128,
    r246 = 246 - 128,
    r247 = 247 - 128,
    r248 = 248 - 128,
    r249 = 249 - 128,
    r250 = 250 - 128,
    r251 = 251 - 128,
    r252 = 252 - 128,
    r253 = 253 - 128,
    r254 = 254 - 128,
    r255 = 255 - 128,
}
pub static R4000ALLEGREX_VFPUCONTROL: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[R4000AllegrexVfpuControl::VFPU_PFXS as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXS", 128 - 128, concat!("$", "128"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_PFXT as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXT", 129 - 128, concat!("$", "129"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_PFXD as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXD", 130 - 128, concat!("$", "130"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_CC as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_CC", 131 - 128, concat!("$", "131"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_INF4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_INF4", 132 - 128, concat!("$", "132"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RSV5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV5", 133 - 128, concat!("$", "133"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RSV6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV6", 134 - 128, concat!("$", "134"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_REV as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_REV", 135 - 128, concat!("$", "135"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX0 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX0", 136 - 128, concat!("$", "136"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX1 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX1", 137 - 128, concat!("$", "137"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX2 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX2", 138 - 128, concat!("$", "138"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX3 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX3", 139 - 128, concat!("$", "139"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX4 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX4", 140 - 128, concat!("$", "140"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX5 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX5", 141 - 128, concat!("$", "141"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX6 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX6", 142 - 128, concat!("$", "142"), false)
    };
    table[R4000AllegrexVfpuControl::VFPU_RCX7 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX7", 143 - 128, concat!("$", "143"), false)
    };
    table[R4000AllegrexVfpuControl::r144 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "144"), 144 - 128, concat!("$", "144"), true)
    };
    table[R4000AllegrexVfpuControl::r145 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "145"), 145 - 128, concat!("$", "145"), true)
    };
    table[R4000AllegrexVfpuControl::r146 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "146"), 146 - 128, concat!("$", "146"), true)
    };
    table[R4000AllegrexVfpuControl::r147 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "147"), 147 - 128, concat!("$", "147"), true)
    };
    table[R4000AllegrexVfpuControl::r148 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "148"), 148 - 128, concat!("$", "148"), true)
    };
    table[R4000AllegrexVfpuControl::r149 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "149"), 149 - 128, concat!("$", "149"), true)
    };
    table[R4000AllegrexVfpuControl::r150 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "150"), 150 - 128, concat!("$", "150"), true)
    };
    table[R4000AllegrexVfpuControl::r151 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "151"), 151 - 128, concat!("$", "151"), true)
    };
    table[R4000AllegrexVfpuControl::r152 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "152"), 152 - 128, concat!("$", "152"), true)
    };
    table[R4000AllegrexVfpuControl::r153 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "153"), 153 - 128, concat!("$", "153"), true)
    };
    table[R4000AllegrexVfpuControl::r154 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "154"), 154 - 128, concat!("$", "154"), true)
    };
    table[R4000AllegrexVfpuControl::r155 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "155"), 155 - 128, concat!("$", "155"), true)
    };
    table[R4000AllegrexVfpuControl::r156 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "156"), 156 - 128, concat!("$", "156"), true)
    };
    table[R4000AllegrexVfpuControl::r157 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "157"), 157 - 128, concat!("$", "157"), true)
    };
    table[R4000AllegrexVfpuControl::r158 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "158"), 158 - 128, concat!("$", "158"), true)
    };
    table[R4000AllegrexVfpuControl::r159 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "159"), 159 - 128, concat!("$", "159"), true)
    };
    table[R4000AllegrexVfpuControl::r160 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "160"), 160 - 128, concat!("$", "160"), true)
    };
    table[R4000AllegrexVfpuControl::r161 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "161"), 161 - 128, concat!("$", "161"), true)
    };
    table[R4000AllegrexVfpuControl::r162 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "162"), 162 - 128, concat!("$", "162"), true)
    };
    table[R4000AllegrexVfpuControl::r163 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "163"), 163 - 128, concat!("$", "163"), true)
    };
    table[R4000AllegrexVfpuControl::r164 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "164"), 164 - 128, concat!("$", "164"), true)
    };
    table[R4000AllegrexVfpuControl::r165 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "165"), 165 - 128, concat!("$", "165"), true)
    };
    table[R4000AllegrexVfpuControl::r166 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "166"), 166 - 128, concat!("$", "166"), true)
    };
    table[R4000AllegrexVfpuControl::r167 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "167"), 167 - 128, concat!("$", "167"), true)
    };
    table[R4000AllegrexVfpuControl::r168 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "168"), 168 - 128, concat!("$", "168"), true)
    };
    table[R4000AllegrexVfpuControl::r169 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "169"), 169 - 128, concat!("$", "169"), true)
    };
    table[R4000AllegrexVfpuControl::r170 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "170"), 170 - 128, concat!("$", "170"), true)
    };
    table[R4000AllegrexVfpuControl::r171 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "171"), 171 - 128, concat!("$", "171"), true)
    };
    table[R4000AllegrexVfpuControl::r172 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "172"), 172 - 128, concat!("$", "172"), true)
    };
    table[R4000AllegrexVfpuControl::r173 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "173"), 173 - 128, concat!("$", "173"), true)
    };
    table[R4000AllegrexVfpuControl::r174 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "174"), 174 - 128, concat!("$", "174"), true)
    };
    table[R4000AllegrexVfpuControl::r175 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "175"), 175 - 128, concat!("$", "175"), true)
    };
    table[R4000AllegrexVfpuControl::r176 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "176"), 176 - 128, concat!("$", "176"), true)
    };
    table[R4000AllegrexVfpuControl::r177 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "177"), 177 - 128, concat!("$", "177"), true)
    };
    table[R4000AllegrexVfpuControl::r178 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "178"), 178 - 128, concat!("$", "178"), true)
    };
    table[R4000AllegrexVfpuControl::r179 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "179"), 179 - 128, concat!("$", "179"), true)
    };
    table[R4000AllegrexVfpuControl::r180 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "180"), 180 - 128, concat!("$", "180"), true)
    };
    table[R4000AllegrexVfpuControl::r181 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "181"), 181 - 128, concat!("$", "181"), true)
    };
    table[R4000AllegrexVfpuControl::r182 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "182"), 182 - 128, concat!("$", "182"), true)
    };
    table[R4000AllegrexVfpuControl::r183 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "183"), 183 - 128, concat!("$", "183"), true)
    };
    table[R4000AllegrexVfpuControl::r184 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "184"), 184 - 128, concat!("$", "184"), true)
    };
    table[R4000AllegrexVfpuControl::r185 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "185"), 185 - 128, concat!("$", "185"), true)
    };
    table[R4000AllegrexVfpuControl::r186 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "186"), 186 - 128, concat!("$", "186"), true)
    };
    table[R4000AllegrexVfpuControl::r187 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "187"), 187 - 128, concat!("$", "187"), true)
    };
    table[R4000AllegrexVfpuControl::r188 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "188"), 188 - 128, concat!("$", "188"), true)
    };
    table[R4000AllegrexVfpuControl::r189 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "189"), 189 - 128, concat!("$", "189"), true)
    };
    table[R4000AllegrexVfpuControl::r190 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "190"), 190 - 128, concat!("$", "190"), true)
    };
    table[R4000AllegrexVfpuControl::r191 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "191"), 191 - 128, concat!("$", "191"), true)
    };
    table[R4000AllegrexVfpuControl::r192 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "192"), 192 - 128, concat!("$", "192"), true)
    };
    table[R4000AllegrexVfpuControl::r193 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "193"), 193 - 128, concat!("$", "193"), true)
    };
    table[R4000AllegrexVfpuControl::r194 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "194"), 194 - 128, concat!("$", "194"), true)
    };
    table[R4000AllegrexVfpuControl::r195 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "195"), 195 - 128, concat!("$", "195"), true)
    };
    table[R4000AllegrexVfpuControl::r196 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "196"), 196 - 128, concat!("$", "196"), true)
    };
    table[R4000AllegrexVfpuControl::r197 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "197"), 197 - 128, concat!("$", "197"), true)
    };
    table[R4000AllegrexVfpuControl::r198 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "198"), 198 - 128, concat!("$", "198"), true)
    };
    table[R4000AllegrexVfpuControl::r199 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "199"), 199 - 128, concat!("$", "199"), true)
    };
    table[R4000AllegrexVfpuControl::r200 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "200"), 200 - 128, concat!("$", "200"), true)
    };
    table[R4000AllegrexVfpuControl::r201 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "201"), 201 - 128, concat!("$", "201"), true)
    };
    table[R4000AllegrexVfpuControl::r202 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "202"), 202 - 128, concat!("$", "202"), true)
    };
    table[R4000AllegrexVfpuControl::r203 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "203"), 203 - 128, concat!("$", "203"), true)
    };
    table[R4000AllegrexVfpuControl::r204 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "204"), 204 - 128, concat!("$", "204"), true)
    };
    table[R4000AllegrexVfpuControl::r205 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "205"), 205 - 128, concat!("$", "205"), true)
    };
    table[R4000AllegrexVfpuControl::r206 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "206"), 206 - 128, concat!("$", "206"), true)
    };
    table[R4000AllegrexVfpuControl::r207 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "207"), 207 - 128, concat!("$", "207"), true)
    };
    table[R4000AllegrexVfpuControl::r208 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "208"), 208 - 128, concat!("$", "208"), true)
    };
    table[R4000AllegrexVfpuControl::r209 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "209"), 209 - 128, concat!("$", "209"), true)
    };
    table[R4000AllegrexVfpuControl::r210 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "210"), 210 - 128, concat!("$", "210"), true)
    };
    table[R4000AllegrexVfpuControl::r211 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "211"), 211 - 128, concat!("$", "211"), true)
    };
    table[R4000AllegrexVfpuControl::r212 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "212"), 212 - 128, concat!("$", "212"), true)
    };
    table[R4000AllegrexVfpuControl::r213 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "213"), 213 - 128, concat!("$", "213"), true)
    };
    table[R4000AllegrexVfpuControl::r214 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "214"), 214 - 128, concat!("$", "214"), true)
    };
    table[R4000AllegrexVfpuControl::r215 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "215"), 215 - 128, concat!("$", "215"), true)
    };
    table[R4000AllegrexVfpuControl::r216 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "216"), 216 - 128, concat!("$", "216"), true)
    };
    table[R4000AllegrexVfpuControl::r217 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "217"), 217 - 128, concat!("$", "217"), true)
    };
    table[R4000AllegrexVfpuControl::r218 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "218"), 218 - 128, concat!("$", "218"), true)
    };
    table[R4000AllegrexVfpuControl::r219 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "219"), 219 - 128, concat!("$", "219"), true)
    };
    table[R4000AllegrexVfpuControl::r220 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "220"), 220 - 128, concat!("$", "220"), true)
    };
    table[R4000AllegrexVfpuControl::r221 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "221"), 221 - 128, concat!("$", "221"), true)
    };
    table[R4000AllegrexVfpuControl::r222 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "222"), 222 - 128, concat!("$", "222"), true)
    };
    table[R4000AllegrexVfpuControl::r223 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "223"), 223 - 128, concat!("$", "223"), true)
    };
    table[R4000AllegrexVfpuControl::r224 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "224"), 224 - 128, concat!("$", "224"), true)
    };
    table[R4000AllegrexVfpuControl::r225 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "225"), 225 - 128, concat!("$", "225"), true)
    };
    table[R4000AllegrexVfpuControl::r226 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "226"), 226 - 128, concat!("$", "226"), true)
    };
    table[R4000AllegrexVfpuControl::r227 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "227"), 227 - 128, concat!("$", "227"), true)
    };
    table[R4000AllegrexVfpuControl::r228 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "228"), 228 - 128, concat!("$", "228"), true)
    };
    table[R4000AllegrexVfpuControl::r229 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "229"), 229 - 128, concat!("$", "229"), true)
    };
    table[R4000AllegrexVfpuControl::r230 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "230"), 230 - 128, concat!("$", "230"), true)
    };
    table[R4000AllegrexVfpuControl::r231 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "231"), 231 - 128, concat!("$", "231"), true)
    };
    table[R4000AllegrexVfpuControl::r232 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "232"), 232 - 128, concat!("$", "232"), true)
    };
    table[R4000AllegrexVfpuControl::r233 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "233"), 233 - 128, concat!("$", "233"), true)
    };
    table[R4000AllegrexVfpuControl::r234 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "234"), 234 - 128, concat!("$", "234"), true)
    };
    table[R4000AllegrexVfpuControl::r235 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "235"), 235 - 128, concat!("$", "235"), true)
    };
    table[R4000AllegrexVfpuControl::r236 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "236"), 236 - 128, concat!("$", "236"), true)
    };
    table[R4000AllegrexVfpuControl::r237 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "237"), 237 - 128, concat!("$", "237"), true)
    };
    table[R4000AllegrexVfpuControl::r238 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "238"), 238 - 128, concat!("$", "238"), true)
    };
    table[R4000AllegrexVfpuControl::r239 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "239"), 239 - 128, concat!("$", "239"), true)
    };
    table[R4000AllegrexVfpuControl::r240 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "240"), 240 - 128, concat!("$", "240"), true)
    };
    table[R4000AllegrexVfpuControl::r241 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "241"), 241 - 128, concat!("$", "241"), true)
    };
    table[R4000AllegrexVfpuControl::r242 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "242"), 242 - 128, concat!("$", "242"), true)
    };
    table[R4000AllegrexVfpuControl::r243 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "243"), 243 - 128, concat!("$", "243"), true)
    };
    table[R4000AllegrexVfpuControl::r244 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "244"), 244 - 128, concat!("$", "244"), true)
    };
    table[R4000AllegrexVfpuControl::r245 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "245"), 245 - 128, concat!("$", "245"), true)
    };
    table[R4000AllegrexVfpuControl::r246 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "246"), 246 - 128, concat!("$", "246"), true)
    };
    table[R4000AllegrexVfpuControl::r247 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "247"), 247 - 128, concat!("$", "247"), true)
    };
    table[R4000AllegrexVfpuControl::r248 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "248"), 248 - 128, concat!("$", "248"), true)
    };
    table[R4000AllegrexVfpuControl::r249 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "249"), 249 - 128, concat!("$", "249"), true)
    };
    table[R4000AllegrexVfpuControl::r250 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "250"), 250 - 128, concat!("$", "250"), true)
    };
    table[R4000AllegrexVfpuControl::r251 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "251"), 251 - 128, concat!("$", "251"), true)
    };
    table[R4000AllegrexVfpuControl::r252 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "252"), 252 - 128, concat!("$", "252"), true)
    };
    table[R4000AllegrexVfpuControl::r253 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "253"), 253 - 128, concat!("$", "253"), true)
    };
    table[R4000AllegrexVfpuControl::r254 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "254"), 254 - 128, concat!("$", "254"), true)
    };
    table[R4000AllegrexVfpuControl::r255 as usize] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "255"), 255 - 128, concat!("$", "255"), true)
    };
    table
};
impl R4000AllegrexVfpuControl {
    pub const fn try_from_u32(value: u32) -> Result<Self, IntRegisterConversionError> {
        match value + 128 {
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
            x => Err(IntRegisterConversionError::new_out_of_range(
                x,
                128,
                "R4000AllegrexVfpuControl",
            )),
        }
    }
    #[must_use]
    pub const fn count() -> usize {
        128
    }
}
impl TryFrom<u32> for R4000AllegrexVfpuControl {
    type Error = IntRegisterConversionError;
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_dollar() {
        for x in &R4000ALLEGREX_VFPUCONTROL {
            if x.has_dollar {
                assert!(
                    x.name.starts_with('$'),
                    "Register {} is missing dollar sign",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| x.starts_with('$')),
                    "Register {:?} is missing dollar sign",
                    x.name_eabi64
                );
            } else {
                assert!(
                    !x.name.starts_with('$'),
                    "Register {} has dollar sign when it shouldn't",
                    x.name
                );
                assert!(
                    x.name_o32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o32
                );
                assert!(
                    x.name_o64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_o64
                );
                assert!(
                    x.name_n32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n32
                );
                assert!(
                    x.name_n64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_n64
                );
                assert!(
                    x.name_eabi32.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi32
                );
                assert!(
                    x.name_eabi64.is_none_or(|x| !x.starts_with('$')),
                    "Register {:?} has dollar sign when it shouldn't",
                    x.name_eabi64
                );
            }
        }
    }
    #[test]
    fn check_descriptor_valid() {
        for (i, x) in R4000ALLEGREX_VFPUCONTROL.iter().enumerate() {
            assert!(x.value() as usize == i, "Broken register index?");
            x.check_valid_entry();
        }
    }
}
