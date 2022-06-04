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
            ['rabbitizer/module.c', 
            "src/instructions/RabbitizerInstr_Disassemble.c", "src/instructions/RabbitizerInstr_ProcessUniqueId.c", "src/instructions/RabbitizerInstr.c", 
            "src/instructions/RabbitizerInstrDescriptor.c", "src/instructions/RabbitizerInstrId.c", "src/instructions/RabbitizerRegister.c",
            "src/common/Utils.c"],
            include_dirs=['include']
        ),
    ],
)
