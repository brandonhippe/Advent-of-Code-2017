import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data, maxLetter = 16):
    """ 2017 Day 16 Part 1

    s1, a spin of size 1: eabcd.
    x3/4, swapping the last two programs: eabdc.
    pe/b, swapping programs e and b: baedc.

    >>> part1(['s1,x3/4,pe/b'], ord('e') - ord('a') + 1)
    'baedc'
    """

    programs = [chr(n + ord('a')) for n in range(maxLetter)]

    for d in data[0].split(','):
        if d[0] == 's':
            n = int(re.findall('\d+', d)[0])
            programs = programs[-n:] + programs[:-n]
        elif d[0] == 'x':
            nums = [int(x) for x in re.findall('\d+', d)]
            programs[nums[0]], programs[nums[1]] = programs[nums[1]], programs[nums[0]]
        elif d[0] == 'p':
            nums = (programs.index(d[1]), programs.index(d[-1]))
            programs[nums[0]], programs[nums[1]] = programs[nums[1]], programs[nums[0]]

    return ''.join(programs)


def part2(data, maxLetter = 16, dances = 1_000_000_000):
    """ 2017 Day 16 Part 2

    >>> part2(['s1,x3/4,pe/b'], ord('e') - ord('a') + 1, 2)
    'ceadb'
    """

    programs = [chr(n + ord('a')) for n in range(maxLetter)]
    
    i = 0
    states = {}
    while i < dances:
        for d in data[0].split(','):
            if d[0] == 's':
                n = int(re.findall('\d+', d)[0])
                programs = programs[-n:] + programs[:-n]
            elif d[0] == 'x':
                nums = [int(x) for x in re.findall('\d+', d)]
                programs[nums[0]], programs[nums[1]] = programs[nums[1]], programs[nums[0]]
            elif d[0] == 'p':
                nums = (programs.index(d[1]), programs.index(d[-1]))
                programs[nums[0]], programs[nums[1]] = programs[nums[1]], programs[nums[0]]

        string = ''.join(programs)

        if string in states:
            i += (1_000_000_000 - i) // (i - states[string]) * (i - states[string])
            
        states[string] = i

        i += 1

    return string


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
        print(f"\nPart 1:\nOrder after dance: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nOrder after a billion dances: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)