import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2017 Day 25 Part 1

    >>> part1(['Begin in state A.', 'Perform a diagnostic checksum after 6 steps.', '', 'In state A:', '  If the current value is 0:', '    - Write the value 1.', '    - Move one slot to the right.', '    - Continue with state B.', '  If the current value is 1:', '    - Write the value 0.', '    - Move one slot to the left.', '    - Continue with state B.', '', 'In state B:', '  If the current value is 0:', '    - Write the value 1.', '    - Move one slot to the left.', '    - Continue with state A.', '  If the current value is 1:', '    - Write the value 1.', '    - Move one slot to the right.', '    - Continue with state A.'])
    3
    """

    state = data[0].split('state ')[1][:-1]
    iterations = int(re.findall('\d+', data[1])[0])

    stateRules = {}
    for i, line in enumerate(data[3:]):        
        if i % 10 == 0:
            currState = line.split('state ')[1][:-1]
            stateRules[currState] = Rule()
        elif i % 10 in [1, 5]:
            currValue = int(re.findall('\d+', line)[0])
        elif i % 10 in [2, 6]:
            stateRules[currState].operations[currValue].append(int(re.findall('\d+', line)[0]))
        elif i % 10 in [3, 7]:
            stateRules[currState].operations[currValue].append(1 if 'right' in line else -1)
        elif i % 10 in [4, 8]:
            stateRules[currState].operations[currValue].append(line.split('state ')[1][:-1])

    tape = defaultdict(lambda: 0)
    cursor = 0
    for _ in range(iterations):
        newVal, cursorInc, newState = stateRules[state].operations[tape[cursor]]
        tape[cursor] = newVal
        cursor += cursorInc
        state = newState

    return sum(tape.values())


def part2(data):
    """ 2017 Day 25 Part 2
    """

    return "Christmas has been saved!"


class Rule:
    def __init__(self):
        self.operations = {0: [], 1: []}


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
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)