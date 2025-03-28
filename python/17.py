import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import deque


def part1(data):
    """ 2017 Day 17 Part 1

    >>> part1(['3'])
    638
    """

    num = int(data[0])
    buf = deque([])

    for i in range(2018):
        buf.rotate(-num)
        buf.append(i)

    return buf[0]


def part2(data):
    """ 2017 Day 17 Part 2
    """

    num = int(data[0]) + 1
    zero_ix = 0
    after_zero = None
    for i in range(1, 50_000_001):
        zero_ix = (zero_ix - num + 1) % i
        if zero_ix == i - 1:
            after_zero = i

    assert after_zero is not None
    return after_zero

    buf = deque([])

    for i in range(50_000_001):
        buf.rotate(-num)
        buf.append(i)

    return buf[(buf.index(0) + 1) % len(buf)]


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
        print(f"\nPart 1:\nNumber after 2017: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber after 50,000,000: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)