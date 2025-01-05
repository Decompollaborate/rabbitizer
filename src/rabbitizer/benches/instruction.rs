/* SPDX-FileCopyrightText: © 2024-2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rabbitizer::{
    display_flags::InstructionDisplayFlags,
    instr::{Instruction, InstructionFlags},
    isa::IsaExtension,
    vram::Vram, IsaVersion,
};

fn decode_none(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(None).with_isa_version(IsaVersion::MIPS_III);

    c.bench_function("decode NONE: nop", |b| {
        b.iter(|| Instruction::new(black_box(0x00000000), vram, flags))
    });
    c.bench_function("decode NONE: jal", |b| {
        b.iter(|| Instruction::new(black_box(0x0C123456), vram, flags))
    });
    c.bench_function("decode NONE: jr", |b| {
        b.iter(|| Instruction::new(black_box(0x03E00008), vram, flags))
    });
    c.bench_function("decode NONE: bltz", |b| {
        b.iter(|| Instruction::new(black_box(0x0440FFF7), vram, flags))
    });
    c.bench_function("decode NONE: cvt.s.w", |b| {
        b.iter(|| Instruction::new(black_box(0x468010A0), vram, flags))
    });
}

fn display_none(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(None).with_isa_version(IsaVersion::MIPS_III);
    let display_flags = InstructionDisplayFlags::new_gnu_as();

    c.bench_function("display NONE: nop", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00000000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display NONE: jal", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0C123456), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display NONE: jr", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x03E00008), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display NONE: bltz", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0440FFF7), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display NONE: cvt.s.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x468010A0), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
}

fn decode_rsp(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::RSP));

    c.bench_function("decode RSP: nop", |b| {
        b.iter(|| Instruction::new(black_box(0x00000000), vram, flags))
    });
    c.bench_function("decode RSP: jal", |b| {
        b.iter(|| Instruction::new(black_box(0x0C123456), vram, flags))
    });
    c.bench_function("decode RSP: jr", |b| {
        b.iter(|| Instruction::new(black_box(0x03E00008), vram, flags))
    });
    c.bench_function("decode RSP: bltz", |b| {
        b.iter(|| Instruction::new(black_box(0x0440FFF7), vram, flags))
    });
    c.bench_function("decode RSP: cvt.s.w", |b| {
        b.iter(|| Instruction::new(black_box(0x468010A0), vram, flags))
    });
    c.bench_function("decode RSP: lqv", |b| {
        b.iter(|| Instruction::new(black_box(0xCA832000), vram, flags))
    });
    c.bench_function("decode RSP: vand", |b| {
        b.iter(|| Instruction::new(black_box(0x4B0C58A8), vram, flags))
    });
    c.bench_function("decode RSP: vrsqh", |b| {
        b.iter(|| Instruction::new(black_box(0x4B1F42F6), vram, flags))
    });
    c.bench_function("decode RSP: ctc2", |b| {
        b.iter(|| Instruction::new(black_box(0x48CB0800), vram, flags))
    });
    c.bench_function("decode RSP: ltv", |b| {
        b.iter(|| Instruction::new(black_box(0xC9085904), vram, flags))
    });
}

fn display_rsp(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::RSP));
    let display_flags = InstructionDisplayFlags::new_gnu_as();

    c.bench_function("display RSP: nop", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00000000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: jal", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0C123456), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: jr", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x03E00008), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: bltz", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0440FFF7), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: cvt.s.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x468010A0), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: lqv", |b| {
        b.iter(|| {
            Instruction::new(black_box(0xCA832000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: vand", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4B0C58A8), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: vrsqh", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4B1F42F6), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: ctc2", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x48CB0800), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display RSP: ltv", |b| {
        b.iter(|| {
            Instruction::new(black_box(0xC9085904), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
}

fn decode_r3000gte(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R3000GTE));

    c.bench_function("decode R3000GTE: nop", |b| {
        b.iter(|| Instruction::new(black_box(0x00000000), vram, flags))
    });
    c.bench_function("decode R3000GTE: jal", |b| {
        b.iter(|| Instruction::new(black_box(0x0C123456), vram, flags))
    });
    c.bench_function("decode R3000GTE: jr", |b| {
        b.iter(|| Instruction::new(black_box(0x03E00008), vram, flags))
    });
    c.bench_function("decode R3000GTE: bltz", |b| {
        b.iter(|| Instruction::new(black_box(0x0440FFF7), vram, flags))
    });
    c.bench_function("decode R3000GTE: cvt.s.w", |b| {
        b.iter(|| Instruction::new(black_box(0x468010A0), vram, flags))
    });
    c.bench_function("decode R3000GTE: RTPS", |b| {
        b.iter(|| Instruction::new(black_box(0x4A180001), vram, flags))
    });
    c.bench_function("decode R3000GTE: RTPT", |b| {
        b.iter(|| Instruction::new(black_box(0x4A280030), vram, flags))
    });
    c.bench_function("decode R3000GTE: SQR", |b| {
        b.iter(|| Instruction::new(black_box(0x4AA00428), vram, flags))
    });
    c.bench_function("decode R3000GTE: OP", |b| {
        b.iter(|| Instruction::new(black_box(0x4B70000C), vram, flags))
    });
    c.bench_function("decode R3000GTE: MVMVA", |b| {
        b.iter(|| Instruction::new(black_box(0x4A486012), vram, flags))
    });
}

fn display_r3000gte(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R3000GTE));
    let display_flags = InstructionDisplayFlags::new_gnu_as();

    c.bench_function("display R3000GTE: nop", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00000000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: jal", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0C123456), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: jr", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x03E00008), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: bltz", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0440FFF7), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: cvt.s.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x468010A0), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: RTPS", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A180001), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: RTPT", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A280030), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: SQR", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4AA00428), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: OP", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4B70000C), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R3000GTE: MVMVA", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A486012), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
}

fn decode_r4000allegrex(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R4000ALLEGREX));

    c.bench_function("decode R4000ALLEGREX: nop", |b| {
        b.iter(|| Instruction::new(black_box(0x00000000), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: jal", |b| {
        b.iter(|| Instruction::new(black_box(0x0C123456), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: jr", |b| {
        b.iter(|| Instruction::new(black_box(0x03E00008), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: bltz", |b| {
        b.iter(|| Instruction::new(black_box(0x0440FFF7), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: cvt.s.w", |b| {
        b.iter(|| Instruction::new(black_box(0x468010A0), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: clo", |b| {
        b.iter(|| Instruction::new(black_box(0x00801017), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: madd", |b| {
        b.iter(|| Instruction::new(black_box(0x00C7001C), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: vwbn.s", |b| {
        b.iter(|| Instruction::new(black_box(0xD3911001), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: vrot.t", |b| {
        b.iter(|| Instruction::new(black_box(0xF3A8801F), vram, flags))
    });
    c.bench_function("decode R4000ALLEGREX: vflush", |b| {
        b.iter(|| Instruction::new(black_box(0xFFFF040D), vram, flags))
    });
}

fn display_r4000allegrex(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R4000ALLEGREX));
    let display_flags = InstructionDisplayFlags::new_gnu_as();

    c.bench_function("display R4000ALLEGREX: nop", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00000000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: jal", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0C123456), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: jr", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x03E00008), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: bltz", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0440FFF7), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: cvt.s.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x468010A0), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: clo", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00801017), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: madd", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00C7001C), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: vwbn.s", |b| {
        b.iter(|| {
            Instruction::new(black_box(0xD3911001), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: vrot.t", |b| {
        b.iter(|| {
            Instruction::new(black_box(0xF3A8801F), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R4000ALLEGREX: vflush", |b| {
        b.iter(|| {
            Instruction::new(black_box(0xFFFF040D), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
}

fn decode_r5900(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R5900));

    c.bench_function("decode R5900: nop", |b| {
        b.iter(|| Instruction::new(black_box(0x00000000), vram, flags))
    });
    c.bench_function("decode R5900: jal", |b| {
        b.iter(|| Instruction::new(black_box(0x0C123456), vram, flags))
    });
    c.bench_function("decode R5900: jr", |b| {
        b.iter(|| Instruction::new(black_box(0x03E00008), vram, flags))
    });
    c.bench_function("decode R5900: bltz", |b| {
        b.iter(|| Instruction::new(black_box(0x0440FFF7), vram, flags))
    });
    c.bench_function("decode R5900: cvt.s.w", |b| {
        b.iter(|| Instruction::new(black_box(0x468010A0), vram, flags))
    });
    c.bench_function("decode R5900: vcallms", |b| {
        b.iter(|| Instruction::new(black_box(0x4A000038), vram, flags))
    });
    c.bench_function("decode R5900: pmthl.lw", |b| {
        b.iter(|| Instruction::new(black_box(0x70000031), vram, flags))
    });
    c.bench_function("decode R5900: vilwr.w", |b| {
        b.iter(|| Instruction::new(black_box(0x4A220BFE), vram, flags))
    });
    c.bench_function("decode R5900: psrlw", |b| {
        b.iter(|| Instruction::new(black_box(0x7011133E), vram, flags))
    });
    c.bench_function("decode R5900: vrnext", |b| {
        b.iter(|| Instruction::new(black_box(0x4A06043C), vram, flags))
    });
}

fn display_r5900(c: &mut Criterion) {
    let vram = Vram::new(0x80000000);
    let flags = InstructionFlags::new().with_isa_extension(Some(IsaExtension::R5900));
    let display_flags = InstructionDisplayFlags::new_gnu_as();

    c.bench_function("display R5900: nop", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x00000000), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: jal", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0C123456), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: jr", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x03E00008), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: bltz", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x0440FFF7), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: cvt.s.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x468010A0), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: vcallms", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A000038), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: pmthl.lw", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x70000031), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: vilwr.w", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A220BFE), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: psrlw", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x7011133E), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
    c.bench_function("display R5900: vrnext", |b| {
        b.iter(|| {
            Instruction::new(black_box(0x4A06043C), vram, flags)
                .display::<&str>(&display_flags, None, 0)
                .to_string()
        })
    });
}

criterion_group!(
    benches,
    decode_none,
    display_none,
    decode_rsp,
    display_rsp,
    decode_r3000gte,
    display_r3000gte,
    decode_r4000allegrex,
    display_r4000allegrex,
    decode_r5900,
    display_r5900
);
criterion_main!(benches);
