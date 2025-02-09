import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data):
    """ 2017 Day 18 Part 1

    >>> part1(['set a 1', 'add a 2', 'mul a a', 'mod a 5', 'snd a', 'set a 0', 'rcv a', 'jgz a -1', 'set a 1', 'jgz a -2'])
    4
    """

    registers = defaultdict(lambda: 0)
    while 0 <= registers['PC'] < len(data):
        ins = data[registers['PC']]
        op, reg, val = ins.split(' ') + [None] * (3 - len(ins.split(' ')))

        if val is not None:
            try:
                val = int(val)
            except ValueError:
                val = registers[val]

        OPSP1[op](reg, val, registers)
        if op == 'rcv' and 'REC' in registers:
            break

        registers['PC'] += 1

    return registers['REC']


def part2(data):
    """ 2017 Day 18 Part 2

    >>> part2(['snd 1', 'snd 2', 'snd p', 'rcv a', 'rcv b', 'rcv c', 'rcv d'])
    3
    """

    regs1 = defaultdict(lambda: 0)
    regs2 = defaultdict(lambda: 0)
    regs2['p'] = 1
    regs1['inQueue'], regs2['inQueue'] = [], []

    currRegs = regs1
    otherRegs = regs2
    
    while True:
        while currRegs['state'] == 0:
            ins = data[currRegs['PC']]
            op, reg, val = ins.split(' ') + [None] * (3 - len(ins.split(' ')))

            if val is not None:
                try:
                    val = int(val)
                except ValueError:
                    val = currRegs[val]

            OPSP2[op](reg, val, currRegs, otherRegs)
            if op == 'rcv' and 'REC' in currRegs:
                break

            currRegs['PC'] += 1

            if not 0 <= currRegs['PC'] < len(data):
                currRegs['state'] = 2

        if 0 not in [currRegs['state'], otherRegs['state']]:
            break

        currRegs, otherRegs = otherRegs, currRegs

    return regs2['sent']


def sound(x, _, regs1, regs2=None):
    try:
        x = int(x)
    except ValueError:
        x = regs1[x]

    regs1['SOUND'] = x


def setr(x, y, regs1, regs2=None):
    regs1[x] = y


def add(x, y, regs1, regs2=None):
    regs1[x] += y


def mul(x, y, regs1, regs2=None):
    regs1[x] *= y


def mod(x, y, regs1, regs2=None):
    regs1[x] %= y


def recov(x, _, regs1, regs2=None):
    try:
        x = int(x)
    except ValueError:
        x = regs1[x]

    if x != 0:
        regs1['REC'] = regs1['SOUND']


def jgz(x, y, regs1, regs2=None):
    try:
        x = int(x)
    except ValueError:
        x = regs1[x]

    if x > 0:
        regs1['PC'] += y - 1


def send(x, y, regs1, regs2=None):
    try:
        x = int(x)
    except ValueError:
        x = regs1[x]

    regs2['inQueue'].append(x)
    if regs2['state'] == 1:
        regs2['state'] = 0

    regs1['sent'] += 1


def receive(x, y, regs1, regs2=None):
    if len(regs1['inQueue']) == 0:
        regs1['PC'] -= 1
        if regs2['state'] != 0:
            regs1['state'] = 2
            regs2['state'] = 2
        else:
            regs1['state'] = 1
    else:
        regs1[x] = regs1['inQueue'].pop(0)
        regs1['state'] = 0


OPSP1 = {'snd': sound, 'set': setr, 'add': add, 'mul': mul, 'mod': mod, 'rcv': recov, 'jgz': jgz}
OPSP2 = {'snd': send, 'set': setr, 'add': add, 'mul': mul, 'mod': mod, 'rcv': receive, 'jgz': jgz}


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
        print(f"\nPart 1:\nRecovered sound: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of values sent by program 1: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)