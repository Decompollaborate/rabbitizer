/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7-6---4--------0
    |  = COP2   |  MFHC2  |                         |1| fmt |       |
    ------6----------5-------------------------------1---3-----------
    |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--| fmt
    | mfvc  | ---   | ---   | ---   | ---   | ---   | ---   | vsync2|
    |-------|-------|-------|-------|-------|-------|-------|-------|
*/

/*
mfvc
    31--------26-25-----21-20-----16-----------------7-6---4-3------0
    |    COP2   |0 0 0 1 1|    rt   |0 0 0 0 0 0 0 0|1|0 0 0|cop2cd_128|
    ------6----------5---------5---------------------1---3------4----
*/

/*
vsync2
    31--------26-25-----21---------------------------7-6---4-3------0
    |    COP2   |0 0 0 1 1|                         |1|1 1 1|1 1 1 1|
    ------6----------5---------5---------------------1---3------4----
*/