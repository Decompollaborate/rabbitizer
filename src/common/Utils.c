/* SPDX-FileCopyrightText: © 2022-2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#include "common/Utils.h"

#include <stdbool.h>
#include <string.h>

int32_t RabbitizerUtils_From2Complement(uint32_t number, int bits) {
    bool isNegative = number & (1ULL << (bits - 1));

    if (isNegative) {
        return -((~number + 1) & ((1ULL << bits) - 1));
    }

    return number;
}

size_t RabbitizerUtils_CharFill(char *dst, int count, char fillchar) {
    if (count <= 0) {
        return 0;
    }

    memset(dst, fillchar, count);

    return count;
}

size_t RabbitizerUtils_escapeString(char *dst, size_t dstSize, const char *src, size_t srcSize) {
    size_t srcPos = 0;
    size_t dstpos = 0;

    for (; srcPos < srcSize && dstpos < dstSize; srcPos++, src++) {
        // The cases of this switch are sorted by ASCII order
        switch (*src) {
            case '\a':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, 'a');
                break;

            case '\t':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, 't');
                break;

            case '\n':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, 'n');
                break;

            case '\f':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, 'f');
                break;

            case '\r':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, 'r');
                break;

            case '"':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '"');
                break;

            case '\\':
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, '\\');
                break;

            default:
                RABUTILS_BUFFER_WRITE_CHAR(dst, dstpos, *src);
                break;
        }
    }

    return dstpos;
}

uint32_t RabbitizerUtils_floatRepr_32From16(uint16_t arg) {
    // IEEE754 16-bit floats are encoded in 16 bits as follows:
    // Sign bit: 1 bit (bit 15)
    // Encoded exponent: 5 bits (bits 10 ~ 15)
    // Fraction/Mantissa: 10 bits (bits 0 ~ 9)

    uint32_t ret;
    int32_t sign;
    int32_t encodedExponent;
    int32_t realExponent;
    bool mantissaIsZero;

    // arg.d = a;

    ret = 0;

    sign = arg >> 15;

    // If parameter is zero, then return zero
    if ((arg & ~(1ULL << 15)) == 0) {
        // Preserve the sign
        ret |= (sign << 31);
        return ret;
    }

    // Clear up the sign
    arg &= ~(1ULL << 15);

    encodedExponent = arg >> 10;
    // Clear up the encoded exponent
    arg &= ~0x7C00ULL;

    // Exponent bias: 0xF
    realExponent = encodedExponent - 0xF;

    mantissaIsZero = (arg == 0);

    if (encodedExponent == 0) {
        // subnormals

        ret |= ((uint32_t)sign) << 31;
        // no need to set the exponent part since it was already zero'd

        // Set the mantissa
        ret |= arg >> (23 - 10);

        return ret;
    }

    if (encodedExponent == 0x1F) {
        // Infinity and NaN

        ret |= ((uint32_t)sign) << 31;
        ret |= 0x7F800000ULL;

        if (!mantissaIsZero) {
            // NaN

            // Set the mantissa to any non-zero value
            ret |= arg << (23 - 10);
        }

        return ret;
    }

    ret |= ((uint32_t)sign) << 31;

    // re-encode the exponent
    ret |= ((uint32_t)(realExponent + 0x7F)) << 23;

    // Set the mantissa
    ret |= arg << (23 - 10);

    return ret;
}
