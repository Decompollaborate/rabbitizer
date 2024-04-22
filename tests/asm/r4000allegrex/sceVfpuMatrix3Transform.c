typedef struct ScePspFVector3 {
    float x, y, z;
} ScePspFVector3;

typedef struct ScePspFMatrix3 {
    ScePspFVector3 x, y, z;
} ScePspFMatrix3;

ScePspFVector3 *sceVfpuMatrix3Transform(ScePspFVector3 *pv0, const ScePspFMatrix3 *pm1, const ScePspFVector3 *pv2)
{
#if 0
    pv0->x = (pm1->x.x * pv2->x) + (pm1->y.x * pv2->y) + (pm1->z.x * pv2->z);
    pv0->y = (pm1->x.y * pv2->x) + (pm1->y.y * pv2->y) + (pm1->z.y * pv2->z);
    pv0->z = (pm1->x.z * pv2->x) + (pm1->y.z * pv2->y) + (pm1->z.z * pv2->z);
#else
    __asm__ (
        ".set            push\n"                    // save assembler option
        ".set            noreorder\n"            // suppress reordering
        "lv.s            s100,  0 + %1\n"        // s100 = pm1->x.x
        "lv.s            s101,  4 + %1\n"        // s101 = pm1->x.y
        "lv.s            s102,  8 + %1\n"        // s102 = pm1->x.z
        "lv.s            s110, 12 + %1\n"        // s110 = pm1->y.x
        "lv.s            s111, 16 + %1\n"        // s111 = pm1->y.y
        "lv.s            s112, 20 + %1\n"        // s112 = pm1->y.z
        "lv.s            s120, 24 + %1\n"        // s120 = pm1->z.x
        "lv.s            s121, 28 + %1\n"        // s121 = pm1->z.y
        "lv.s            s122, 32 + %1\n"        // s122 = pm1->z.z
        "lv.s            s200,  0 + %2\n"        // s200 = pv2->x
        "lv.s            s201,  4 + %2\n"        // s201 = pv2->y
        "lv.s            s202,  8 + %2\n"        // s202 = pv2->z
        "vctfm3.t        c000, e100, c200\n"        // c000 = e100 * c200
        "sv.s            s000,  0 + %0\n"        // pv0->x = s000
        "sv.s            s001,  4 + %0\n"        // pv0->y = s001
        "sv.s            s002,  8 + %0\n"        // pv0->z = s002
        ".set            pop\n"                    // restore assembler option
        : "=o"(*pv0)
        : "o"(*pm1), "o"(*pv2)
    );
#endif
    return (pv0);
}
