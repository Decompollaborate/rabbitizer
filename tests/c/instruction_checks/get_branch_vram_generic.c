/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "rabbitizer.h"

#include <stdlib.h>

int main() {
    RabbitizerInstruction inst;
    RabbitizerInstruction_init(&inst, 0x14600001, 0x80000000); // bnez        $v1, . + 4 + (0x1 << 2)
    RabbitizerInstruction_processUniqueId(&inst);
    if (RabbitizerInstruction_getBranchVramGeneric(&inst) < 0) {
        printf("Word '0x%08X' has a negative branch target\n", inst.word);
        return 1;
    }
    return 0;
}
