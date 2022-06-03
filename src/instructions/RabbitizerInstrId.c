#include "instructions/RabbitizerInstrId.h"

#define RABBITIZER_DEF_STR_ARRAY(prefix, x) \
    [prefix##x] = #x

const char *RabbitizerInstrCpuId_Names[RABBITIZER_INSTR_ID_MAX] = {
    RABBITIZER_DEF_STR_ARRAY(RABBITIZER_INSTR_CPU_ID_, INVALID),
};
