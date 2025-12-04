from itertools import batched
p1 = 0
p2 = 0
with open("input/day02.txt") as file:
    for rng in file.read().split(","):
        [s, e] = [int(v) for v in rng.split("-")]
        for v in range(s, e + 1):
            sv = str(v)
            lsv = len(sv)
            mid = lsv // 2
            if ((lsv % 2) == 0) and (sv[:mid] == sv[mid:]):
                p1 += v
            for w in range(1, mid+1):
                if len(set(batched(sv, w))) == 1:
                    p2 += v
                    break
print(p1, p2)
