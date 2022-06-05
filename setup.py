# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from setuptools import setup, find_packages, Extension


setup(
    name='rabbitizer',
    version='0.1.0',
    packages=find_packages(),
    ext_modules=[
        Extension(
            # the qualified name of the extension module to build
            'rabbitizer',
            # the files to compile into our module relative to ``setup.py``
            ["rabbitizer/rabbitizer_module.c", "rabbitizer/rabbitizer_type_Instr.c", "rabbitizer/rabbitizer_submodule_instr_id.c",
            "src/instructions/RabbitizerInstr/RabbitizerInstr_Disassemble.c", "src/instructions/RabbitizerInstr/RabbitizerInstr_ProcessUniqueId.c", "src/instructions/RabbitizerInstr/RabbitizerInstr.c", "src/instructions/RabbitizerInstr/RabbitizerInstr_Examination.c",
            "src/instructions/RabbitizerInstrDescriptor.c", "src/instructions/RabbitizerInstrId.c", "src/instructions/RabbitizerRegister.c",
            "src/common/Utils.c", "src/common/RabbitizerConfig.c"],
            include_dirs=['include']
        ),
    ],
)
