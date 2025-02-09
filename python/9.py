import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2017 Day 9 Part 1

    {}, score of 1.
    {{{}}}, score of 1 + 2 + 3 = 6.
    {{},{}}, score of 1 + 2 + 2 = 5.
    {{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
    {<a>,<a>,<a>,<a>}, score of 1.
    {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
    {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.

    >>> part1(['{}'])
    1
    >>> part1(['{{{}}}'])
    6
    >>> part1(['{{},{}}'])
    5
    >>> part1(['{{{},{},{{}}}}'])
    16
    >>> part1(['{<a>,<a>,<a>,<a>}'])
    1
    >>> part1(['{{<ab>},{<ab>},{<ab>},{<ab>}}'])
    9
    >>> part1(['{{<!!>},{<!!>},{<!!>},{<!!>}}'])
    9
    >>> part1(['{{<a!>},{<a!>},{<a!>},{<ab>}}'])
    3
    """

    text = data[0]
    while re.search('!.', text):
        text = re.sub('!.', '', text, 1)

    removed = 0
    while '<' in text:
        start, end = text.index('<'), text.index('>') + 1
        text = text[:start] + text[end:]
        removed += end - start - 2

    g = Group(text[1:-1])

    return g.score()


def part2(data):
    """ 2017 Day 9 Part 2

    <>, 0 characters.
    <random characters>, 17 characters.
    <<<<>, 3 characters.
    <{!>}>, 2 characters.
    <!!>, 0 characters.
    <!!!>>, 0 characters.
    <{o"i!a,<{i<a>, 10 characters.

    >>> part2(['<>'])
    0
    >>> part2(['<random characters>'])
    17
    >>> part2(['<<<<>'])
    3
    >>> part2(['<{!>}>'])
    2
    >>> part2(['<!!>'])
    0
    >>> part2(['<!!!>>'])
    0
    >>> part2(['<{o"i!a,<{i<a>'])
    10
    """

    text = data[0]
    while re.search('!.', text):
        text = re.sub('!.', '', text, 1)

    removed = 0
    while '<' in text:
        start, end = text.index('<'), text.index('>') + 1
        text = text[:start] + text[end:]
        removed += end - start - 2

    return removed


class Group:
    def __init__(self, gText, gNum = 0):
        self.num = gNum + 1
        self.subGroups = []

        start = re.search('{', gText)
        while re.search('{', gText):
            start = start.span()[0] + 1
            end = start
            opened = 1
            while opened != 0:
                if gText[end] == '{':
                    opened += 1
                elif gText[end] == '}':
                    opened -= 1

                end += 1

            end -= 1             

            self.subGroups.append(Group(gText[start:end], self.num))
            gText = gText[end + 1:]
            start = re.search('{', gText)

    def score(self):
        return sum([self.num] + [s.score() for s in self.subGroups])


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
        print(f"\nPart 1:\nScore: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nGarbage removed: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)