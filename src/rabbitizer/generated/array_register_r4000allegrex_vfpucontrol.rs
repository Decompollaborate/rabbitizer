/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/* Automatically generated. DO NOT MODIFY */

use crate::{RegisterDescriptor, RegisterR4000AllegrexVfpuControl};
pub static R4000ALLEGREX_VFPUCONTROL_REGISTERS: [RegisterDescriptor; 128] = {
    let mut table = [RegisterDescriptor::default(); 128];
    table[RegisterR4000AllegrexVfpuControl::VFPU_PFXS as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXS", 128, concat!("$", "128"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_PFXT as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXT", 129, concat!("$", "129"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_PFXD as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_PFXD", 130, concat!("$", "130"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_CC as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_CC", 131, concat!("$", "131"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_INF4 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_INF4", 132, concat!("$", "132"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RSV5 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV5", 133, concat!("$", "133"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RSV6 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RSV6", 134, concat!("$", "134"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_REV as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_REV", 135, concat!("$", "135"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX0 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX0", 136, concat!("$", "136"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX1 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX1", 137, concat!("$", "137"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX2 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX2", 138, concat!("$", "138"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX3 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX3", 139, concat!("$", "139"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX4 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX4", 140, concat!("$", "140"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX5 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX5", 141, concat!("$", "141"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX6 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX6", 142, concat!("$", "142"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::VFPU_RCX7 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new("VFPU_RCX7", 143, concat!("$", "143"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r144 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "144"), 144, concat!("$", "144"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r145 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "145"), 145, concat!("$", "145"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r146 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "146"), 146, concat!("$", "146"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r147 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "147"), 147, concat!("$", "147"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r148 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "148"), 148, concat!("$", "148"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r149 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "149"), 149, concat!("$", "149"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r150 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "150"), 150, concat!("$", "150"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r151 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "151"), 151, concat!("$", "151"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r152 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "152"), 152, concat!("$", "152"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r153 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "153"), 153, concat!("$", "153"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r154 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "154"), 154, concat!("$", "154"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r155 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "155"), 155, concat!("$", "155"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r156 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "156"), 156, concat!("$", "156"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r157 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "157"), 157, concat!("$", "157"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r158 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "158"), 158, concat!("$", "158"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r159 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "159"), 159, concat!("$", "159"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r160 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "160"), 160, concat!("$", "160"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r161 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "161"), 161, concat!("$", "161"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r162 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "162"), 162, concat!("$", "162"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r163 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "163"), 163, concat!("$", "163"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r164 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "164"), 164, concat!("$", "164"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r165 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "165"), 165, concat!("$", "165"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r166 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "166"), 166, concat!("$", "166"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r167 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "167"), 167, concat!("$", "167"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r168 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "168"), 168, concat!("$", "168"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r169 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "169"), 169, concat!("$", "169"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r170 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "170"), 170, concat!("$", "170"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r171 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "171"), 171, concat!("$", "171"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r172 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "172"), 172, concat!("$", "172"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r173 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "173"), 173, concat!("$", "173"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r174 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "174"), 174, concat!("$", "174"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r175 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "175"), 175, concat!("$", "175"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r176 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "176"), 176, concat!("$", "176"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r177 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "177"), 177, concat!("$", "177"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r178 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "178"), 178, concat!("$", "178"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r179 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "179"), 179, concat!("$", "179"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r180 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "180"), 180, concat!("$", "180"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r181 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "181"), 181, concat!("$", "181"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r182 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "182"), 182, concat!("$", "182"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r183 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "183"), 183, concat!("$", "183"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r184 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "184"), 184, concat!("$", "184"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r185 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "185"), 185, concat!("$", "185"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r186 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "186"), 186, concat!("$", "186"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r187 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "187"), 187, concat!("$", "187"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r188 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "188"), 188, concat!("$", "188"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r189 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "189"), 189, concat!("$", "189"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r190 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "190"), 190, concat!("$", "190"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r191 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "191"), 191, concat!("$", "191"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r192 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "192"), 192, concat!("$", "192"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r193 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "193"), 193, concat!("$", "193"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r194 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "194"), 194, concat!("$", "194"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r195 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "195"), 195, concat!("$", "195"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r196 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "196"), 196, concat!("$", "196"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r197 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "197"), 197, concat!("$", "197"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r198 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "198"), 198, concat!("$", "198"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r199 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "199"), 199, concat!("$", "199"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r200 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "200"), 200, concat!("$", "200"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r201 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "201"), 201, concat!("$", "201"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r202 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "202"), 202, concat!("$", "202"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r203 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "203"), 203, concat!("$", "203"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r204 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "204"), 204, concat!("$", "204"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r205 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "205"), 205, concat!("$", "205"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r206 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "206"), 206, concat!("$", "206"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r207 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "207"), 207, concat!("$", "207"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r208 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "208"), 208, concat!("$", "208"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r209 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "209"), 209, concat!("$", "209"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r210 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "210"), 210, concat!("$", "210"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r211 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "211"), 211, concat!("$", "211"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r212 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "212"), 212, concat!("$", "212"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r213 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "213"), 213, concat!("$", "213"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r214 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "214"), 214, concat!("$", "214"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r215 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "215"), 215, concat!("$", "215"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r216 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "216"), 216, concat!("$", "216"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r217 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "217"), 217, concat!("$", "217"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r218 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "218"), 218, concat!("$", "218"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r219 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "219"), 219, concat!("$", "219"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r220 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "220"), 220, concat!("$", "220"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r221 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "221"), 221, concat!("$", "221"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r222 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "222"), 222, concat!("$", "222"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r223 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "223"), 223, concat!("$", "223"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r224 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "224"), 224, concat!("$", "224"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r225 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "225"), 225, concat!("$", "225"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r226 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "226"), 226, concat!("$", "226"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r227 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "227"), 227, concat!("$", "227"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r228 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "228"), 228, concat!("$", "228"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r229 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "229"), 229, concat!("$", "229"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r230 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "230"), 230, concat!("$", "230"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r231 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "231"), 231, concat!("$", "231"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r232 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "232"), 232, concat!("$", "232"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r233 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "233"), 233, concat!("$", "233"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r234 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "234"), 234, concat!("$", "234"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r235 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "235"), 235, concat!("$", "235"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r236 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "236"), 236, concat!("$", "236"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r237 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "237"), 237, concat!("$", "237"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r238 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "238"), 238, concat!("$", "238"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r239 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "239"), 239, concat!("$", "239"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r240 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "240"), 240, concat!("$", "240"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r241 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "241"), 241, concat!("$", "241"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r242 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "242"), 242, concat!("$", "242"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r243 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "243"), 243, concat!("$", "243"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r244 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "244"), 244, concat!("$", "244"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r245 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "245"), 245, concat!("$", "245"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r246 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "246"), 246, concat!("$", "246"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r247 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "247"), 247, concat!("$", "247"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r248 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "248"), 248, concat!("$", "248"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r249 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "249"), 249, concat!("$", "249"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r250 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "250"), 250, concat!("$", "250"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r251 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "251"), 251, concat!("$", "251"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r252 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "252"), 252, concat!("$", "252"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r253 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "253"), 253, concat!("$", "253"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r254 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "254"), 254, concat!("$", "254"))
    }
    .check_panic_chain();
    table[RegisterR4000AllegrexVfpuControl::r255 as usize - 128] = RegisterDescriptor {
        ..RegisterDescriptor::new(concat!("$", "255"), 255, concat!("$", "255"))
    }
    .check_panic_chain();
    let mut i = 0;
    while i < 128 {
        assert!(table[i].value as usize - 128 == i);
        i += 1;
    }
    table
};
