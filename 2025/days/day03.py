p1 = 0
p2 = 0
with open("input/day03.txt") as file:
    for line in (ln.strip() for ln in file.readlines()):
        l, r = 0, 1
        ml, mr = "0", "0"
        for i in range(1, len(line)):
            if (line[l] < line[i]) and (i < len(line) - 1):
                l = i
                r = l + 1
            elif line[r] < line[i]:
                r = i
        p1 += int(line[l] + line[r])
        best = []
        bi = 0
        for v in range(11, -1, -1):
            bi, b = max(
                enumerate(line[bi : (len(line) - v)], bi + 1), key=lambda x: x[1]
            )
            best.append(b)
        p2 += int("".join(best))
print(p1, p2)
