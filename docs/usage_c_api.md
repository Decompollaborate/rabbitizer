# Usage of the C API

## Simple example to disassemble a word

```c
#include "rabbitizer.h"
#include <stdlib.h>

int main() {
    RabbitizerInstruction instr;
    uint32_t word = 0x8D4A7E18;
    uint32_t vram = 0x80000000;
    char *buffer;
    size_t bufferSize;

    RabbitizerInstruction_init(&instr, word, vram);
    RabbitizerInstruction_processUniqueId(&instr);

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, 0, 0);
    buffer = malloc(bufferSize + 1);

    RabbitizerInstruction_disassemble(&instr, buffer, NULL, 0, 0);

    printf("%s\n", buffer);

    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}
```

Compiling and running the above C code prints the following:

```mips
lw          $t2, 0x7E18($t2)
```

Please note many safe-guards were removed from this example for simplicity, like
checking if `malloc` returned a `NULL` pointer.

Let's break up the example and explain each part:

1. The stack

The `RabbitizerInstruction` type is the type `rabbitizer` uses to represent an
instruction. It is a simple struct which doesn't need dynamic memory
allocation of any kind, so it can be declared as an automatic variable and live
in the stack, without worrying about pointers and such.

The other stack variables should be self-explanatory. `word` is a 32-bit word
representing a raw MIPS instruction (spoiler, it is an `lw`). `rabbitizer`
needs to know the `vram` address of the instruction it is decoding, so we
initialize with a place-holder one. `buffer` and `bufferSize` will be used for
storing the disassembled string.

```c
int main() {
    RabbitizerInstruction instr;
    uint32_t word = 0x8D4A7E18;
    uint32_t vram = 0x80000000;
    char *buffer;
    size_t bufferSize;
```

2. Initializing

To initialize our `instr` we need to call the pair `RabbitizerInstruction_init`
and `RabbitizerInstruction_processUniqueId`. `RabbitizerInstruction_init`
initialises all the members of the struct so it doesn't contain garbage data
anymore, while `RabbitizerInstruction_processUniqueId` does the heavy lifting of
identifying the actual instruction id out of the `word` we passed.

A `RabbitizerInstruction` variable is not considered fully initialized until it
has been passed to this pair of functions.

```c
    RabbitizerInstruction_init(&instr, word, vram);
    RabbitizerInstruction_processUniqueId(&instr);
```

3. Disassembling into a string

To disassemble the passed word as a string we can call
`RabbitizerInstruction_disassemble`. This function expects a `char` buffer to
fill, which should have enough space to hold the resulting string. To know how
big this buffer needs to be we should use the
`RabbitizerInstruction_getSizeForBuffer` function which calculates a size big
enough to hold the disassembled string for the passed instruction (without
taking into account the finalizing NUL character, similar to how `strlen`
behaves).

With this information we can just `malloc` our buffer and call
`RabbitizerInstruction_disassemble` to get our disassembled instruction.

(Ignore the extra `0` and `NULL` arguments for now, they will be discussed later)

```c
    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, 0, 0);
    buffer = malloc(bufferSize + 1);

    RabbitizerInstruction_disassemble(&instr, buffer, NULL, 0, 0);
```

4. Printing

Not much to say here, just print the disassembled instruction to `stdout`.

```c
    printf("%s\n", buffer);
```

5. Clean-up

Finally since we know we won't be using the produced string or the instruction
we just `free` and `RabbitizerInstruction_destroy` them.

As a curiosity, `RabbitizerInstruction_destroy` currently does nothing, but
exists in case some destruction is needed in the future, so it recommended to
call this function as a future-proof method.

```c
    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}
```

## Overriding the immediate

When disassembling an instruction which has an immediate you'll probably don't
want raw immediate embedded in the disassembled string, but instead it reference
a symbol passed by the user.

For example, instead of having the raw immediate here

```mips
lw          $t2, 0x7E18($t2)
bnez        $t1, . + 4 + (-0x5 << 2)
```

You may want to reference symbols like this

```mips
lw          $t2, %lo(some_symbol)($t2)
bnez        $t1, some_branch_label
```

To do this you need to simply pass the string which will override the immediate
to the `immOverride` parameter of the `RabbitizerInstruction_disassemble`
function. For example:

```c
#include "rabbitizer.h"
#include <stdlib.h>
#include <string.h>

int main() {
    RabbitizerInstruction instr;
    uint32_t word = 0x8D4A7E18; // lw          $t2, 0x7E18($t2)
    uint32_t vram = 0x80000000;
    char *buffer;
    size_t bufferSize;
    const char *immOverride = "%lo(some_symbol)";
    size_t immOverrideLength;

    immOverrideLength = strlen(immOverride);

    RabbitizerInstruction_init(&instr, word, vram);
    RabbitizerInstruction_processUniqueId(&instr);

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, immOverrideLength, 0);
    buffer = malloc(bufferSize + 1);

    RabbitizerInstruction_disassemble(&instr, buffer, immOverride, immOverrideLength, 0);

    printf("%s\n", buffer);

    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}
```

`RabbitizerInstruction_disassemble` will do the heavy lifting of using the
passed string as immediate so the user doesn't have to do string manipulations
to replace it.

Note both `RabbitizerInstruction_getSizeForBuffer` and
`RabbitizerInstruction_disassemble` require the length of the override string,
it can be easily computed with `strlen`.

Also note the passed `immOverride` includes the `%lo` reloc operand.
`rabbitizer` does not perform any kind of logic to add reloc operands, that kind
of logic is expected to be handled by the user, the library will use the
`immOverride` blindly.

Finally, `immOverride` and `immOverrideLength` must be both `NULL`/`0` or both
point to a valid NUL-terminated string and its size, using a combination of them
(as in non-`NULL` and `0` length or `NULL` and non-`zero` length) is UB.

For completeness sake, here's the code to produce the `bnez` from the above
example.

```c
#include "rabbitizer.h"
#include <stdlib.h>
#include <string.h>

int main() {
    RabbitizerInstruction instr;
    uint32_t word = 0x1520FFFB; // bnez        $t1, . + 4 + (-0x5 << 2)
    uint32_t vram = 0x80000000;
    char *buffer;
    size_t bufferSize;
    const char *immOverride = "some_branch_label";
    size_t immOverrideLength;

    immOverrideLength = strlen(immOverride);

    RabbitizerInstruction_init(&instr, word, vram);
    RabbitizerInstruction_processUniqueId(&instr);

    bufferSize = RabbitizerInstruction_getSizeForBuffer(&instr, immOverrideLength, 0);
    buffer = malloc(bufferSize + 1);

    RabbitizerInstruction_disassemble(&instr, buffer, immOverride, immOverrideLength, 0);

    printf("%s\n", buffer);

    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}
```
