/* SPDX-FileCopyrightText: © 2024 Decompollaborate */
/* SPDX-License-Identifier: MIT */

/*
    31--------26-25------21 -------------------------7--------------0
    |  = COP2   |  MFHC2  |                         |p|             |
    ------6----------5-------------------------------1---------------
    |---------------0---------------|---------------1---------------| p
    | mtv                           | mtvc                          |
    |-------------------------------|-------------------------------|
*/

/*
mtv
    31--------26-25-----21-20-----16-----------------7-6------------0
    |    COP2   |0 0 1 1 1|    rt   |0 0 0 0 0 0 0 0|0|      vd     |
    ------6----------5---------5---------------------1-------7-------
*/

/*
mtvc
    31--------26-25-----21-20-----16-----------------7-6---4-3------0
    |    COP2   |0 0 1 1 1|    rt   |0 0 0 0 0 0 0 0|1|0 0 0|cop2cd_128|
    ------6----------5---------5---------------------1---3------4----
*/