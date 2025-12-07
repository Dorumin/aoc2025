#!/usr/bin/env python3


########################################################################################################################
# Imports
########################################################################################################################

from collections.abc import Iterable
from dataclasses import dataclass


########################################################################################################################
# Diagram
########################################################################################################################

EMPTY_SPACE = '.'
PAPER_ROLL = '@'

@dataclass
class Chiagram:
    width: int
    height: int
    cells: list[bool]

    @classmethod
    def from_lines(cls, lines: Iterable[str]) -> 'Chiagram':
        width = -1
        cells: list[bool] = []

        for y, line in enumerate(lines):
            width = len(line) + 2

            if y == 0:
                cells.extend([False] * width)

            cells.append(False)

            cells.extend(
                char == PAPER_ROLL
                for char in line
            )

            cells.append(False)

        cells.extend([False] * width)

        height = y + 3 # type: ignore I guess it's never empty

        return cls(width, height, cells)

    def prune(self, max_adjacent_paper_rolls: int) -> int:
        height = self.height
        width = self.width
        pruned = 0

        # No need to clone I guess
        # new_cells = [cell for cell in self.cells] # DON'T edit it in place

        offsets = [(-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1)]

        for y in range(1, height - 1):
            row_base = y * width

            for x in range(1, width - 1):
                idx = row_base + x

                if not self.cells[idx]:
                    continue

                adjacent = 0

                for dx, dy in offsets:
                    nx = x + dx
                    ny = y + dy

                    # assert 0 <= nx < width and 0 <= ny < height

                    if self.cells[ny * width + nx]:
                        adjacent += 1

                        if adjacent > max_adjacent_paper_rolls:
                            break

                if adjacent <= max_adjacent_paper_rolls:
                    self.cells[idx] = False
                    pruned += 1

        return pruned

    def prune_until(self, max_adjacent_paper_rolls: int) -> int:
        total = 0

        while True:
            pruned = self.prune(max_adjacent_paper_rolls)

            if pruned == 0:
                break

            total += pruned

        return total


########################################################################################################################
# Part 1
########################################################################################################################

MAX_ADJACENT_PAPER_ROLLS = 3

def count_accessible_paper_rolls(lines: Iterable[str]) -> int:
    """
    >>> count_accessible_paper_rolls([
    ...     '..@@.@@@@.',
    ...     '@@@.@.@.@@',
    ...     '@@@@@.@.@@',
    ...     '@.@@@@..@.',
    ...     '@@.@@@@.@@',
    ...     '.@@@@@@@.@',
    ...     '.@.@.@.@@@',
    ...     '@.@@@.@@@@',
    ...     '.@@@@@@@@.',
    ...     '@.@.@@@.@.',
    ... ])
    13
    """
    return Chiagram.from_lines(lines).prune(MAX_ADJACENT_PAPER_ROLLS)


########################################################################################################################
# Part 2
########################################################################################################################

def count_potentially_accessible_paper_rolls(lines: Iterable[str]) -> int:
    """
    >>> count_potentially_accessible_paper_rolls([
    ...     '..@@.@@@@.',
    ...     '@@@.@.@.@@',
    ...     '@@@@@.@.@@',
    ...     '@.@@@@..@.',
    ...     '@@.@@@@.@@',
    ...     '.@@@@@@@.@',
    ...     '.@.@.@.@@@',
    ...     '@.@@@.@@@@',
    ...     '.@@@@@@@@.',
    ...     '@.@.@@@.@.',
    ... ])
    43
    """
    diagram = Chiagram.from_lines(lines)
    accessible_paper_rolls = diagram.prune_until(MAX_ADJACENT_PAPER_ROLLS)
    return accessible_paper_rolls


########################################################################################################################
# CLI bootstrap
########################################################################################################################

def main() -> None:
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument('part', type=int, choices=(1, 2))
    parser.add_argument('input', type=argparse.FileType('rt'))
    args = parser.parse_args()
    lines = (line.rstrip('\n') for line in args.input)

    if args.part == 1:
        print(count_accessible_paper_rolls(lines))
    elif args.part == 2:
        print(count_potentially_accessible_paper_rolls(lines))
    else:
        raise ValueError(f'{args.part} is not a valid part')


if __name__ == '__main__':
    main()
