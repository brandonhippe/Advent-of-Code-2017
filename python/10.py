import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data, elements = 256):
    """ 2017 Day 10 Part 1

    >>> part1(['3, 4, 1, 5'], 5)
    12
    """

    lengths = [int(x) for x in data[0].split(',')]
    nums = list(range(elements))

    currPos = 0
    for ss, l in enumerate(lengths):
        if l != 0:
            nums = nums[currPos:] + nums[:currPos]
            nums = nums[l - 1::-1] + nums[l:]
            nums = nums[-currPos:] + nums[:-currPos]

        currPos += ss + l
        currPos %= len(nums)

    return nums[0] * nums[1]


def part2(data):
    """ 2017 Day 10 Part 2


    The empty string becomes a2582a3a0e66e6e86e3812dcb672a272.
    AoC 2017 becomes 33efeb34ea91902bb2f59c9920caa6cd.
    1,2,3 becomes 3efbe78a8d82f29979031a4aa0b16a9d.
    1,2,4 becomes 63960835bcdc130f0b66d7ff4f6a5a8e.

    >>> part2([''])
    'a2582a3a0e66e6e86e3812dcb672a272'
    >>> part2(['AoC 2017'])
    '33efeb34ea91902bb2f59c9920caa6cd'
    >>> part2(['1,2,3'])
    '3efbe78a8d82f29979031a4aa0b16a9d'
    >>> part2(['1,2,4'])
    '63960835bcdc130f0b66d7ff4f6a5a8e'
    """

    bs = [ord(x) for x in data[0]] + [17, 31, 73, 47, 23]
    nums = list(range(256))
    
    ss = 0
    currPos = 0
    for _ in range(64):
        for l in bs:
            if l != 0:
                nums = nums[currPos:] + nums[:currPos]
                nums = nums[l - 1::-1] + nums[l:]
                nums = nums[-currPos:] + nums[:-currPos]

            currPos += ss + l
            currPos %= len(nums)
            ss += 1

    denseHash = []
    for i, n in enumerate(nums):
        if i % 16 == 0:
            denseHash.append(0)

        denseHash[-1] = denseHash[-1] ^ n

    return ''.join(['0' * (2 - len(hex(n)[2:])) + hex(n)[2:] for n in denseHash])


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
        print(f"\nPart 1:\nProduct of first two numbers in list: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nKnot Hash: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)