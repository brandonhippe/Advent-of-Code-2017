import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2017 Day 22 Part 1

    >>> part1(['..#', '#..', '...'])
    5587
    """

    infected = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                infected.add((x, y))

    pos = (len(data[0]) // 2, len(data) // 2)
    facing = (0, -1)
    infectedBursts = 0
    for _ in range(10_000):
        if pos in infected:
            facing = (-facing[1], facing[0])
            infected.remove(pos)
        else:
            facing = (facing[1], -facing[0])
            infected.add(pos)
            infectedBursts += 1

        pos = tuple(p + o for p, o in zip(pos, facing))

    return infectedBursts


def part2(data):
    """ 2017 Day 22 Part 2

    >>> part2(['..#', '#..', '...'])
    2511944
    """

    infected = set()

    for y, line in enumerate(data):
        for x, l in enumerate(line):
            if l == '#':
                infected.add((x, y))

    pos = (len(data[0]) // 2, len(data) // 2)
    facing = (0, -1)
    infectedBursts = 0
    weakened = set()
    flagged = set()
    for i in range(10_000_000):            
        if pos in infected:
            facing = (-facing[1], facing[0])
            infected.remove(pos)
            flagged.add(pos)
        elif pos in weakened:
            weakened.remove(pos)
            infected.add(pos)
            infectedBursts += 1
        elif pos in flagged:
            facing = (-facing[0], -facing[1])
            flagged.remove(pos)
        else:
            facing = (facing[1], -facing[0])
            weakened.add(pos)

        pos = tuple(p + o for p, o in zip(pos, facing))

    return infectedBursts


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
        print(f"\nPart 1:\nBursts that cause an infection: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nBursts that cause an infection: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)