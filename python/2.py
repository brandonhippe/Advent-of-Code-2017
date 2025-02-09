import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2017 Day 2 Part 1

    >>> part1(['5 1 9 5', '7 5 3', '2 4 6 8'])
    18
    """

    lines = [[int(x) for x in re.findall('-?\d+', line)] for line in data]

    return sum([max(line) - min(line) for line in lines])


def part2(data):
    """ 2017 Day 2 Part 2

    >>> part2(['5 9 2 8', '9 4 7 3', '3 8 6 5'])
    9
    """

    lines = [[int(x) for x in re.findall('-?\d+', line)] for line in data]

    return sum([div(line) for line in lines])


def div(line):
    for d1 in line:
        for d2 in line:
            if d1 != d2:
                if d1 % d2 == 0:
                    return d1 // d2
                if d2 % d1 == 0:
                    return d2 // d1


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
        print(f"\nPart 1:\nChecksum: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nDivsum: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)