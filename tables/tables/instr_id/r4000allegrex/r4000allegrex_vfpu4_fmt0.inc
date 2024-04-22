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
     *1 = See FMT0 table
     *2 = See FMT2 table
     *3 = See FMT3 table
     *4 = See RND table
     *5 = See CVTFLT table
     *6 = See CVTINT table
     *7 = See FMT8 table
     *8 = See FMT9 table
     *9 = See CONTROL table
     *10 = See COLOR table
     *11 = See CST table
*/
