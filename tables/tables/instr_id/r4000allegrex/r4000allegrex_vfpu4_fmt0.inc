/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25--23-----19--------------------------------------0
    |   VFPU4   |0 0|   fmt   |                                     |
    ------6-------2------5-------------------------------------------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| lo
 00 | *1    | ---   | *2    |  *3   | *4    | ---   | *5    | *6    |
 01 | *7    | *8    | *9    | *10   | *11   | *11   | *11   | *11   |
 10 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 11 | ---   | ---   | ---   | ---   | ---   | ---   | ---   | ---   |
 hi |-------|-------|-------|-------|-------|-------|-------|-------|
     *1 = See FMT0 FMT0 FMT0 table
     *2 = See FMT0 FMT0 FMT2 table
     *3 = See FMT0 FMT0 FMT3 table
     *4 = See FMT0 FMT1 FMT0 table
     *5 = See FMT0 FMT1 FMT2 table
     *6 = See FMT0 FMT1 FMT3 table
     *7 = See FMT0 FMT2 FMT0 table
     *8 = See FMT0 FMT2 FMT1 table
     *9 = See FMT0 FMT2 FMT2 table
     *10 = See FMT0 FMT2 FMT3 table
     *11 = See FMT0 FMT3 table
*/
