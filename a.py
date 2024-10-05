from pathlib import Path

p = Path("tables/tables/opcodes/r4000allegrex/r4000allegrex_vfpu4_fmt0_color.inc")

lines = []

with p.open() as f:
    for line in f:
        if "r4000allegrex, " in line:
            thingy = line.split(", ")
            val = int(thingy[1], 16)
            a = val & ~0x101
            b = val & 0x101
            val = (a >> 1) | b
            thingy[1] = f"0x{val:04X}"
            line = ", ".join(thingy)
        lines.append(line)

with p.open("w") as f:
    for line in lines:
        f.write(line)
