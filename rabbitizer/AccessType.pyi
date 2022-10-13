#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from .Enum import Enum


class AccessType:
    INVALID: Enum

    BYTE_SIGNED: Enum
    BYTE_UNSIGNED: Enum
    SHORT_SIGNED: Enum
    SHORT_UNSIGNED: Enum
    WORD_SIGNED: Enum
    WORD_UNSIGNED: Enum
    DOUBLEWORD_SIGNED: Enum
    DOUBLEWORD_UNSIGNED: Enum
    FLOAT_SIGNED: Enum
    DOUBLEFLOAT_SIGNED: Enum

    MAX: Enum
