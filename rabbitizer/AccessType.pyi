#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations

from .Enum import Enum


class AccessType:
    INVALID: Enum

    BYTE: Enum
    SHORT: Enum
    WORD: Enum
    DOUBLEWORD: Enum
    FLOAT: Enum
    DOUBLEFLOAT: Enum

    MAX: Enum
