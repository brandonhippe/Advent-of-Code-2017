import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2017 Day 3 Part 1

    Data from square 1 is carried 0 steps, since it's at the access port.
    Data from square 12 is carried 3 steps, such as: down, left, left.
    Data from square 23 is carried only 2 steps: up twice.
    Data from square 1024 must be carried 31 steps.

    >>> part1(['1'])
    0
    >>> part1(['12'])
    3
    >>> part1(['23'])
    2
    >>> part1(['1024'])
    31
    """

    return manhatDist(sqIndexPos(int(data[0])), (0, 0))


def part2(data):
    """ 2017 Day 3 Part 2
    """

    generated = defaultdict(lambda: 0)
    generated[(0, 0)] = 1
    lastGenerated = 1
    pos = [1, 0]
    dirs = [[[0, -1], 1], [[-1, 0], 2], [[0, 1], 2], [[1, 0], 3]]
    
    while lastGenerated < int(data[0]):
        offset, d = dirs.pop(0)

        for _ in range(d):
            lastGenerated = sum([generated[tuple(p + o for p, o in zip(pos, off))] for off in [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]]])
            if lastGenerated > int(data[0]):
                return lastGenerated

            generated[tuple(pos)] = lastGenerated

            pos = [p + o for p, o in zip(pos, offset)]

        dirs.append([offset, d + 2])

    return -1


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


def sqIndexPos(sq):
    odd = 1
    while odd * odd <= sq:
        odd += 2

    odd -= 2
    pos = [odd // 2] * 2
    ix = odd * odd

    if ix == sq:
        return pos

    pos[0] += 1
    ix += 1
    dirs = [[[0, -1], odd], [[-1, 0], odd + 1], [[0, 1], odd + 1], [[1, 0], odd + 1]]
    while sq - ix > dirs[0][-1]:
        offset, d = dirs.pop(0)
        ix += d
        pos = [p + (d * o) for p, o in zip(pos, offset)]

    offset, d = dirs[0]

    return [p + ((sq - ix) * o) for p, o in zip(pos, offset)]


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nSteps to access port: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFirst value written larger than input: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)