This folder is a small test suite to check how the official SN toolchain
encodes and decodes instructions, so we can properly mimic it.

The `.dump.s` files are the decoded files by the `snbin.exe` tool.

To add more cases to the existing files then add them to the `.s` files and run
`make COMPILER_PATH=path/to/psp/sn/toolchain`. This will use `wine` by default.
The `wine` path can be changing by using the setting the `WINE=` variable when
invoking the makefile.
