import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2017 Day 12 Part 1

    >>> part1(['0 <-> 2', '1 <-> 1', '2 <-> 0, 3, 4', '3 <-> 2, 4', '4 <-> 2, 3, 6', '5 <-> 6', '6 <-> 4, 5'])
    6
    """

    programs = [Program(i) for i in range(len(data))]

    for i, line in enumerate(data):
        ps = [int(x) for x in re.findall('\d+', line.split('> ')[1])]
        for p in ps:
            programs[i].pipes.add(programs[p])
            programs[p].pipes.add(programs[i])

    visited = [False] * len(programs)
    bfs(programs[0], visited)

    return sum(visited)


def part2(data):
    """ 2017 Day 12 Part 2

    >>> part2(['0 <-> 2', '1 <-> 1', '2 <-> 0, 3, 4', '3 <-> 2, 4', '4 <-> 2, 3, 6', '5 <-> 6', '6 <-> 4, 5'])
    2
    """

    programs = [Program(i) for i in range(len(data))]

    for i, line in enumerate(data):
        ps = [int(x) for x in re.findall('\d+', line.split('> ')[1])]
        for p in ps:
            programs[i].pipes.add(programs[p])
            programs[p].pipes.add(programs[i])

    visited = [False] * len(programs)
    bfs(programs[0], visited)

    groups = 1
    for i, v in enumerate(visited):
        if not v:
            bfs(programs[i], visited)
            groups += 1

    return groups


class Program:
    def __init__(self, pNum):
        self.pNum = pNum
        self.pipes = set()


def bfs(start, visited):
    openList = [start]

    while len(openList) != 0:
        p = openList.pop(0)
        for s in p.pipes:
            if not visited[s.pNum]:
                openList.append(s)

        visited[p.pNum] = True


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
        print(f"\nPart 1:\nPrograms connected to program 0: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of groups: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)