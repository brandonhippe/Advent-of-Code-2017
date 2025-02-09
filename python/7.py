import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2017 Day 7 Part 1

    >>> part1(['pbga (66)', 'xhth (57)', 'ebii (61)', 'havc (66)', 'ktlj (57)', 'fwft (72) -> ktlj, cntj, xhth', 'qoyq (66)', 'padx (45) -> pbga, havc, qoyq', 'tknk (41) -> ugml, padx, fwft', 'jptl (61)', 'ugml (68) -> gyxo, ebii, jptl', 'gyxo (61)', 'cntj (57)'])
    'tknk'
    """

    programs = [Program(line) for line in data]

    pNames = {p.name for p in programs}
    for p in programs:
        for s in p.subPrograms:
            if s in pNames:
                pNames.remove(s)

    return list(pNames)[0]


def part2(data):
    """ 2017 Day 7 Part 2

    >>> part2(['pbga (66)', 'xhth (57)', 'ebii (61)', 'havc (66)', 'ktlj (57)', 'fwft (72) -> ktlj, cntj, xhth', 'qoyq (66)', 'padx (45) -> pbga, havc, qoyq', 'tknk (41) -> ugml, padx, fwft', 'jptl (61)', 'ugml (68) -> gyxo, ebii, jptl', 'gyxo (61)', 'cntj (57)'])
    60
    """

    programs = [Program(line.strip('\n')) for line in data]

    pNames = {p.name for p in programs}
    for p in programs:
        for s in p.subPrograms:
            if s in pNames:
                pNames.remove(s)

    head = list(pNames)[0]

    programs = {p.name: p for p in programs}
    for p in programs.values():
        for i, s in enumerate(p.subPrograms):
            p.subPrograms[i] = programs[s]

    return programs[head].balance()


class Program:
    def __init__(self, programText):
        self.weight = int(re.findall('\d+', programText)[0])
        self.name = programText.split(' ')[0]
        self.subPrograms = []
        if '->' in programText:
            self.subPrograms = programText.split('-> ')[1].split(', ')

    def genWeight(self):
        return sum([self.weight] + [s.genWeight() for s in self.subPrograms])

    def balance(self):
        subWeights = defaultdict(lambda: [])
        for s in self.subPrograms:
            subWeights[s.genWeight()].append(s)

        for w, s in zip(subWeights.keys(), subWeights.values()):
            if len(s) == 1:
                subBalanced = s[0].balance()
                if subBalanced == 0:
                    ow = list(o for o in subWeights.keys() if o != w)[0]
                    return s[0].weight + ow - w
                else:
                    return subBalanced

        return 0


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
        print(f"\nPart 1:\nName of bottom program: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nWeight of unbalanced program needed to balance: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)