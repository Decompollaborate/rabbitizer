# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from setuptools import setup, Extension
from pathlib import Path

bindingsPath = Path("rabbitizer")
srcPath = Path("src")

sourcesList = [str(x) for x in bindingsPath.glob("**/*.c")] + [str(x) for x in srcPath.glob("**/*.c")]

setup(
    ext_modules=[
        Extension(
            name="rabbitizer",
            sources=sourcesList,
            include_dirs=["include", "rabbitizer"],
            extra_compile_args = [
                "-std=c11",
                "-Wall",
                "-g",
            ],
        ),
    ],
)
