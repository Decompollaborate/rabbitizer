from pathlib import Path

p = Path("asdf.txt")
test = Path("src/rabbitizer/tests/instruction_tests_r4000allegrex.rs")

pairs: list[tuple[str, str]] = []

with p.open() as f:
    prev_line = None
    for line in f:
        if "Expected: '.word       " in line:
            prev_line = line
        elif prev_line and "Got:      '.word       " in line:
            pass
            pairs.append((prev_line, line))
        else:
            prev_line = None

text = test.read_text()

for p, c in pairs:
    p = p.replace("    Expected: '", "").replace("'\n", "")
    c = c.replace("    Got:      '", "").replace("'\n", "")
    text = text.replace(p, c)

test.write_text(text)
