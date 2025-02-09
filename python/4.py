import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
def part1(data):
    """ 2017 Day 4 Part 1

    aa bb cc dd ee is valid.
    aa bb cc dd aa is not valid - the word aa appears more than once.
    aa bb cc dd aaa is valid - aa and aaa count as different words.

    >>> part1(['aa bb cc dd ee', 'aa bb cc dd aa', 'aa bb cc dd aaa'])
    2
    """

    lines = [line.split(' ') for line in data]

    return sum([1 if len(set(l for l in line)) == len(line) else 0 for line in lines])


def part2(data):
    """ 2017 Day 4 Part 2

    abcde fghij is a valid passphrase.
    abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the first word.
    a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming another word.
    iiii oiii ooii oooi oooo is valid.
    oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.


    >>> part2(['abcde fghij', 'abcde xyz ecdab', 'a ab abc abd abf abj', 'iiii oiii ooii oooi oooo', 'oiii ioii iioi iiio'])
    3
    """

    total = 0
    lines = [line.split(' ') for line in data]

    for line in lines:
        words = set()
        for w in line:
            d = tuple(sorted(((c, w.count(c)) for c in w), key=lambda e: e[0]))
            if d not in words:
                words.add(d)
        
        if len(words) == len(line):
            total += 1

    return total


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
        print(f"\nPart 1:\nValid passphrases: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValid passphrases: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)