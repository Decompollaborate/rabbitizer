#!/usr/bin/env python3

# SPDX-FileCopyrightText: © 2022 Decompollaborate
# SPDX-License-Identifier: MIT

from __future__ import annotations


class LoPairingInfo:
    shouldProcess: bool
    instrOffset: int
    isGpRel: bool
    isGpLoad: bool

    def __init__(self): ...
