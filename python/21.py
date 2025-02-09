import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import numpy as np


def part1(data, iterations = 5):
    """ 2017 Day 21 Part 1

    >>> part1(['../.# => ##./#../...', '.#./..#/### => #..#/..../..../#..#'], 2)
    12
    """

    rules = {l[0]: np.array([list(line) for line in l[1].split('/')]) for l in [line.strip('\n').split(' => ') for line in data]}

    tempRules = rules.copy()

    for rule in tempRules.keys():
        r = np.array([list(line) for line in rule.split('/')])
        for i in range(7):
            r = np.flipud(r)
            if i % 2 == 1:
                r = np.rot90(r)

            rules['/'.join(''.join(line) for line in r)] = rules[rule]

    img = np.array([list(line) for line in ['.#.', '..#', '###']])

    for _ in range(iterations):
        sz = 2 if len(img) % 2 == 0 else 3
        newImg = None
        for y in range(0, len(img), sz):
            lines = None
            for x in range(0, len(img[y]), sz):
                group = rules['/'.join(''.join(line) for line in img[y:y+sz, x:x+sz])]

                if lines is None:
                    lines = group
                else:
                    lines = np.concatenate((lines, group), axis=1)

            if newImg is None:
                newImg = lines
            else:
                newImg = np.concatenate((newImg, lines), axis=0)

        img = newImg

    return sum(sum(np.char.count(img, '#')))


def part2(data):
    """ 2017 Day 21 Part 2
    """

    rules = {l[0]: np.array([list(line) for line in l[1].split('/')]) for l in [line.strip('\n').split(' => ') for line in data]}

    tempRules = rules.copy()

    for rule in tempRules.keys():
        r = np.array([list(line) for line in rule.split('/')])
        for i in range(7):
            r = np.flipud(r)
            if i % 2 == 1:
                r = np.rot90(r)

            rules['/'.join(''.join(line) for line in r)] = rules[rule]

    img = np.array([list(line) for line in ['.#.', '..#', '###']])

    for _ in range(18):
        sz = 2 if len(img) % 2 == 0 else 3
        newImg = None
        for y in range(0, len(img), sz):
            lines = None
            for x in range(0, len(img[y]), sz):
                group = rules['/'.join(''.join(line) for line in img[y:y+sz, x:x+sz])]

                if lines is None:
                    lines = group
                else:
                    lines = np.concatenate((lines, group), axis=1)

            if newImg is None:
                newImg = lines
            else:
                newImg = np.concatenate((newImg, lines), axis=0)

        img = newImg

    return sum(sum(np.char.count(img, '#')))


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
        print(f"\nPart 1:\nNumber of lit pixels after 5 iterations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of lit pixels after 18 iterations: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)