import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import defaultdict


def part1(data):
    """ 2017 Day 23 Part 1
    """

    registers = defaultdict(lambda: 0)
    multiplies = 0
    while 0 <= registers['PC'] < len(data):
        ins = data[registers['PC']]
        op, reg, val = ins.split(' ') + [None] * (3 - len(ins.split(' ')))

        if val is not None:
            try:
                val = int(val)
            except ValueError:
                val = registers[val]

        multiplies += OPS[op](reg, val, registers)
        registers['PC'] += 1

    return multiplies


def part2(data):
    """ 2017 Day 23 Part 2
    """

    registers = defaultdict(lambda: 0)
    registers['a'] = 1
    while registers['PC'] < 8:
        ins = data[registers['PC']]
        op, reg, val = ins.split(' ') + [None] * (3 - len(ins.split(' ')))

        if val is not None:
            try:
                val = int(val)
            except ValueError:
                val = registers[val]

        OPS[op](reg, val, registers)
        registers['PC'] += 1

    h = 0
    stepSize = int(re.findall('\d+', data[-2])[0])
    b, c = registers['b'], registers['c']
    primes = SieveOfEratosthenes(c)

    for n in range(b, c+1, stepSize):
        if not primes[n]:
            h += 1

    return h



def setr(x, y, regs):
    regs[x] = y
    return 0


def sub(x, y, regs):
    regs[x] -= y
    return 0


def mul(x, y, regs):
    regs[x] *= y
    return 1


def jnz(x, y, regs):
    try:
        x = int(x)
    except ValueError:
        x = regs[x]

    if x != 0:
        regs['PC'] += y - 1

    return 0


OPS = {'set': setr, 'sub': sub, 'mul': mul, 'jnz': jnz}


def SieveOfEratosthenes(n):
    primes = defaultdict(lambda: True)
    p = 2
    while (p * p <= n):
        if (primes[p] == True):
            for i in range(p ** 2, n + 1, p):
                primes[i] = False
        p += 1
    primes[0]= False
    primes[1]= False

    return primes


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
        print(f"\nPart 1:\nNumber of multiplies: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValue in h: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)