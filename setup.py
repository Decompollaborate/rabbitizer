# SPDX-FileCopyrightText: © 2022-2024 Decompollaborate
# SPDX-License-Identifier: MIT

from setuptools import setup, Extension
import sysconfig
from pathlib import Path
import platform

Py_GIL_DISABLED = sysconfig.get_config_var("Py_GIL_DISABLED")

bindingsPath = Path("rabbitizer")
srcPath = Path("src")

sourcesList = [str(x) for x in bindingsPath.glob("**/*.c")] + [str(x) for x in srcPath.glob("**/*.c")]
headersList = [str(x) for x in bindingsPath.glob("**/*.h")] + [str(x) for x in srcPath.glob("**/*.h")]

extraCompileArgs = ["-std=c11", "-Wall", "-g",]
if platform.system() == "Linux":
    extraCompileArgs += ["-Wno-unknown-warning-option",]
    extraCompileArgs += ["-Os", "-Wextra",]
    extraCompileArgs += ["-Werror=vla", "-Werror=switch", "-Werror=implicit-fallthrough", "-Werror=unused-function", "-Werror=unused-parameter", "-Werror=shadow", "-Werror=switch"]
    extraCompileArgs += ["-Werror=implicit-function-declaration", "-Werror=incompatible-pointer-types"]
    extraCompileArgs += ["-Werror"]
    extraCompileArgs += ["-Wno-nonnull-compare"]

# Only enable abi3 build if we are not building a free threaded wheel.
define_macros = []
py_limited_api = False
options = {}
if not Py_GIL_DISABLED:
    define_macros = [
        ("Py_LIMITED_API", "0x03040000"),
    ]
    py_limited_api = True
    options={"bdist_wheel": {"py_limited_api": "cp34"}}

setup(
    ext_modules=[
        Extension(
            name="rabbitizer",
            sources=sourcesList,
            include_dirs=["include", "rabbitizer", "tables"],
            define_macros=define_macros,
            extra_compile_args = extraCompileArgs,
            depends=headersList,
            py_limited_api=py_limited_api,
        ),
    ],
    options=options,
)
