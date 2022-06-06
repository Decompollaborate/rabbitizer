# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from setuptools import setup, find_packages, Extension


with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name='rabbitizer',
    version='0.1.0',
    author="Decompollaborate",
    license="MIT",
    url="https://github.com/Decompollaborate/rabbitizer",
    description="MIPS instruction disassembler",
    long_description=long_description,
    long_description_content_type="text/markdown",
    # packages=find_packages(),
    package_data={"rabbitizer": ["py.typed", "rabbitizer.pyi"]},
    zip_safe=False,
    ext_modules=[
        Extension(
            # the qualified name of the extension module to build
            "rabbitizer",
            # the files to compile into our module relative to ``setup.py``
            ["rabbitizer/rabbitizer_module.c", "rabbitizer/rabbitizer_type_Instr.c", "rabbitizer/rabbitizer_global_instr_id.c", "rabbitizer/rabbitizer_global_config.c",
            "src/instructions/RabbitizerInstr/RabbitizerInstr_Disassemble.c", "src/instructions/RabbitizerInstr/RabbitizerInstr_ProcessUniqueId.c", "src/instructions/RabbitizerInstr/RabbitizerInstr.c", "src/instructions/RabbitizerInstr/RabbitizerInstr_Examination.c",
            "src/instructions/RabbitizerInstrDescriptor.c", "src/instructions/RabbitizerInstrId.c", "src/instructions/RabbitizerRegister.c",
            "src/common/Utils.c", "src/common/RabbitizerConfig.c"],
            include_dirs=["include"],
            extra_compile_args = [
                "-std=c11",
                "-Wall",
                "-Wextra",
                # "-Wpedantic", # variadic macros warning :s
                "-Werror=implicit-function-declaration",
                "-Werror=implicit-function-declaration",
                "-Werror=incompatible-pointer-types",
                "-Werror=vla",
                "-Werror=switch",
                "-Werror=implicit-fallthrough",
                "-Werror=unused-function",
                "-Werror=shadow",
                # "-Werror",
                "-g",
            ],
        ),
    ],
)
