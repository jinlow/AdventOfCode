from functools import reduce
p1 = 0
p2 = 0
with open("input/day03.txt") as file:
    for line in (ln.strip() for ln in file.readlines()):
        l, r = 0, 1
        ml, mr = '0', '0'
        for i in range(1, len(line)):
            if (line[l] < line[i]) and (i < len(line) - 1):
                l = i
                r = l + 1
            elif line[r] < line[i]:
                r = i
        p1 += int(line[l] + line[r])
        best = []
        for v in range(0, 12):
            ...

print(p1)        