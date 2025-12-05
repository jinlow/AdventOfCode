import itertools
import copy
p1 = 0
p2 = 0
with open("input/day04.txt") as file:
    data = [list(ln.strip()) for ln in file.readlines()]
for row in range(0, len(data)):
    for col in range(0, len(data[row])):
        if data[row][col] != "@":
            continue
        paper = 0
        for rs, cs in itertools.product(*[[-1, 1, 0]]*2):
            if ((rs, cs) != (0, 0)) and ((rs + row) >= 0) and ((cs + col) >= 0):
                try:
                    paper += (data[rs + row][cs + col] == "@")
                except IndexError:
                    continue
        if paper < 4:
            p1 += 1

cleared = False
while not cleared:
    cleared = True
    for row in range(0, len(data)):
        for col in range(0, len(data[row])):
            if data[row][col] != "@":
                continue
            paper = 0
            for rs, cs in itertools.product(*[[-1, 1, 0]]*2):
                if ((rs, cs) != (0, 0)) and ((rs + row) >= 0) and ((cs + col) >= 0):
                    try:
                        paper += (data[rs + row][cs + col] == "@")
                    except IndexError:
                        continue
            if paper < 4:
                p2 += 1
                data[row][col] = "."
                cleared = False

# import numpy as np
# print(np.array(data_c))
# print(np.array(data))
print(p1, p2)
