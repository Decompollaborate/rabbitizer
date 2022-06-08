# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from setuptools import setup, Extension


setup(
    ext_modules=[
        Extension(
            "rabbitizer",
            ["rabbitizer/rabbitizer_module.c", "rabbitizer/rabbitizer_type_Instruction.c", "rabbitizer/rabbitizer_global_instr_id.c", "rabbitizer/rabbitizer_global_config.c",
            "src/instructions/RabbitizerInstruction/RabbitizerInstruction_Disassemble.c", "src/instructions/RabbitizerInstruction/RabbitizerInstruction_ProcessUniqueId.c", "src/instructions/RabbitizerInstruction/RabbitizerInstruction.c", "src/instructions/RabbitizerInstruction/RabbitizerInstruction_Examination.c",
            "src/instructions/RabbitizerInstructionRsp/RabbitizerInstructionRsp.c", "src/instructions/RabbitizerInstructionRsp/RabbitizerInstructionRsp_ProcessUniqueId.c",
            "src/instructions/RabbitizerInstrDescriptor.c", "src/instructions/RabbitizerInstrId.c", "src/instructions/RabbitizerRegister.c",
            "src/common/Utils.c", "src/common/RabbitizerConfig.c"],
            include_dirs=["include"],
            extra_compile_args = [
                "-std=c11",
                "-Wall",
                "-Wextra",
                # "-Wpedantic", # binary constants :s
                "-Wno-cast-function-type",
                "-Werror=implicit-function-declaration",
                "-Werror=implicit-function-declaration",
                "-Werror=incompatible-pointer-types",
                "-Werror=vla",
                "-Werror=switch",
                "-Werror=implicit-fallthrough",
                "-Werror=unused-function",
                "-Werror=unused-parameter",
                "-Werror=shadow",
                # "-Werror",
                "-g",
            ],
        ),
    ],
)
