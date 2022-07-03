/* SPDX-FileCopyrightText: © 2022 Decompollaborate */
/* SPDX-License-Identifier: MIT */

#ifndef RABBITIZER_UTILS_H
#define RABBITIZER_UTILS_H

#include <stddef.h>
#include <stdint.h>


#if !defined(__GNUC__) && !defined(__clang__)
#define __attribute__(x)
#endif

#if __STDC_VERSION__ >= 202300L
#define DEPRECATED(reason) [[deprecated (reason)]]
#define FALLTHROUGH [[fallthrough]]
#define NODISCARD(reason) [[nodiscard (reason)]]
#define NORETURN [[noreturn]]
#define UNUSED [[maybe_unused]]
#else
#define DEPRECATED(reason) __attribute__((deprecated (reason)))
#define FALLTHROUGH __attribute__((fallthrough))
#define NODISCARD(reason) __attribute__((warn_unused_result))
#define NORETURN _Noreturn
#define UNUSED __attribute__((unused))
#endif

#if defined(_MSC_VER)
#  define UNREACHABLE __assume(0)
#else
#  define UNREACHABLE __builtin_unreachable()
#endif

#define PURE __attribute__((pure))


#define ARRAY_COUNT(arr) (sizeof(arr) / sizeof(arr[0]))

#define MASK(v, w) ((v) & ((1 << (w)) - 1))

/*
 * the SHIFT macros take a value, a shift amount, and a width.
 *
 * For the left shift, the lower bits of the value are masked,
 * then shifted left.
 *
 * For the right shift, the value is shifted right, then the lower bits
 * are masked.
 *
 * (NOTE: SHIFTL(v, 0, 32) won't work, just use an assignment)
 *
 */
#define SHIFTL(v, s, w)	(((v) & ((1 << (w)) - 1)) << (s))
#define SHIFTR(v, s, w)	(((v) >> (s)) & ((1 << (w)) - 1))

#define BITREPACK(fullword, v, s, w) ((SHIFTR((fullword), (s)+(w), 32-((s)+(w))) << ((s)+(w))) | SHIFTL((v), (s), (w)) | MASK((fullword), (s)))
#define BITREPACK_RIGHT(fullword, v, s, w) (SHIFTL((v), (s), (w)) | MASK((fullword), (s)))


int32_t RabbitizerUtils_From2Complement(uint32_t number, int bits);
size_t RabbitizerUtils_CharFill(char *dst, int count, char fillchar);

#endif
