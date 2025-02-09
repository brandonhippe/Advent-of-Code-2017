import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data):
    """ 2017 Day 20 Part 1

    >>> part1(['p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>', 'p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>'])
    0
    """

    particles = [Particle([int(x) for x in re.findall('-?\d+', line)], i) for i, line in enumerate(data)]

    closest = particles[particles.index(closestParticle(particles)) - 1]
    i = 0
    while i % 1000 != 0 or closest != closestParticle(particles):
        closest = closestParticle(particles)
        for p in particles:
            p.update()

        i += 1

    return closest.ix


def part2(data):
    """ 2017 Day 20 Part 2

    >>> part2(['p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>', 'p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>', 'p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>', 'p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>'])
    1
    """

    particles = [Particle([int(x) for x in re.findall('-?\d+', line)], i) for i, line in enumerate(data)]

    pLen = len(particles) + 1
    i = 0

    while i % 1000 != 0 or len(particles) != pLen:
        pLen = len(particles)
        poses = set()
        deleting = set()

        for p in particles:
            p.update()

            if tuple(p.pos) in poses:
                poses.remove(tuple(p.pos))
                deleting.add(tuple(p.pos))
            elif tuple(p.pos) not in deleting:
                poses.add(tuple(p.pos))

        newParticles = []
        for p in particles:
            if tuple(p.pos) not in deleting:
                newParticles.append(p)

        particles = newParticles
        i += 1

    return len(particles)


class Particle:
    def __init__(self, nums, ix):
        self.pos = nums[:3]
        self.vel = nums[3:6]
        self.acc = nums[6:]
        self.ix = ix

    def update(self):
        self.vel = [v + a for v, a in zip(self.vel, self.acc)]
        self.pos = [p + v for p, v in zip(self.pos, self.vel)]


def manhatDist(p1, p2):
    return sum([abs(c1 - c2) for c1, c2 in zip(p1, p2)])


def closestParticle(particles):
    minD = float('inf')
    for p in particles:
        d = manhatDist(p.pos, (0, 0, 0))
        if d < minD:
            minD = d
            minP = p

    return minP


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
        print(f"\nPart 1:\nClosest particle: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of particles after resolving collisions: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return [(p1, p1_time.elapsed), (p2, p2_time.elapsed)]


if __name__ == "__main__":
    main(verbose=True)