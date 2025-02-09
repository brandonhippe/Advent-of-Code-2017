import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2017 Day 11 Part 1

    ne,ne,ne is 3 steps away.
    ne,ne,sw,sw is 0 steps away (back where you started).
    ne,ne,s,s is 2 steps away (se,se).
    se,sw,se,sw,sw is 3 steps away (s,s,sw).

    >>> part1(['ne,ne,ne'])
    3
    >>> part1(['ne,ne,sw,sw'])
    0
    >>> part1(['ne,ne,s,s'])
    2
    >>> part1(['se,sw,se,sw,sw'])
    3
    """

    steps = data[0].split(',')

    pos = (0, 0)

    for step in steps:
        pos = tuple(p + o for p, o in zip(pos, MOVES[step]))

        d = manhatDist((*pos, -pos[0] - pos[1]), (0, 0, 0)) // 2

    return d


def part2(data):
    """ 2017 Day 11 Part 2
    """
    
    steps = data[0].split(',')

    pos = (0, 0)

    farthest = 0
    for step in steps:
        pos = tuple(p + o for p, o in zip(pos, MOVES[step]))

        d = manhatDist((*pos, -pos[0] - pos[1]), (0, 0, 0)) // 2
        if d > farthest:
            farthest = d

    return farthest

MOVES = {'n': (0, -1), 'ne': (1, -1), 'se': (1, 0), 's': (0, 1), 'sw': (-1, 1), 'nw': (-1, 0)}


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


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
        print(f"\nPart 1:\nFewest steps to child process: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nFarthest ever reached: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)