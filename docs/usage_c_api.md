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

To produce a string disassembly of the word we passed we could simply call to
`RabbitizerInstruction_disassemble`. This function expects a `char` buffer to
fill, which should have enough space to hold the resulting string. To know how
big this buffer needs to be we should use the
`RabbitizerInstruction_getSizeForBuffer` function which calculates a size big
enough to hold the disassembled string for the passed instruction (without
taking into account the finalizing NUL character).

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

```c
    free(buffer);
    RabbitizerInstruction_destroy(&instr);

    return 0;
}
```
